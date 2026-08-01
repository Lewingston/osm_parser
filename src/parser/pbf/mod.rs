
use crate::map::{
    MapData
};

mod error;

use error::Error;
use error::BlobError;

mod osm_block;

pub use osm_block::BlockData;

mod string_table;

#[allow(warnings)]
#[allow(clippy::all)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/osmpbf/mod.rs"));
}

use protos::fileformat::BlobHeader;
use protos::fileformat::Blob;
use protos::fileformat::blob;
use protobuf::Message;

use crate::parser;


enum BlobType {
    OsmHeader,
    OsmData
}


struct BlobInfo {
    data_size: usize,
    type_:     BlobType
}


pub struct PbfParserResult {
    pub map:    MapData,
    pub blocks: Vec<BlockData>
}


impl Default for PbfParserResult {

    fn default() -> Self {
        Self {
            map:    MapData::create_empty_map(),
            blocks: Vec::<BlockData>::new()
        }
    }
}


/// # Errors
///
/// Returns an error when parsing of the file failed.
pub fn from_file(file_name: &str) -> Result<PbfParserResult, Box<dyn std::error::Error>> {

    let file   = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);

    parse(reader)
}



fn parse<R: std::io::Read>(mut reader: R) -> Result<PbfParserResult, Box<dyn std::error::Error>> {

    let mut blocks = Vec::<BlockData>::new();

    let mut blob_count = 0;

    loop {

        let mut buffer = [0; 4];
        match reader.read_exact(&mut buffer) {
            Ok(()) => {

                let header_size = u32::from_be_bytes(buffer) as usize;
                println!("Blob: {blob_count}");
                blocks.append(&mut read_blob(&mut reader, header_size, blob_count)?);
                blob_count += 1;
                println!();

            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                _ => { return Err(Box::new(err)); }
            }
        }
    }

    let map = create_map(&blocks);

    Ok(PbfParserResult {
        map,
        blocks
    })
}


fn read_blob<R: std::io::Read>(
    reader:      &mut R,
    header_size: usize,
    blob_num:    usize
) -> Result<Vec<BlockData>, Box<dyn std::error::Error>> {

    let blob_info = read_blob_header(reader, header_size, blob_num)?;

    let blob_data = read_blob_data(reader, blob_info.data_size)?;

    match blob_info.type_ {
        BlobType::OsmHeader => {
            parse_osm_header(&blob_data)?;
            Ok(Vec::<BlockData>::new())
        }
        BlobType::OsmData   => {
            osm_block::parse(&blob_data)
        }
    }
}


fn read_blob_header<R: std::io::Read>(
    reader:   &mut R,
    size:     usize,
    blob_num: usize
) -> Result<BlobInfo, Box<dyn std::error::Error>> {

    let mut buffer = vec![0u8; size];
    reader.read_exact(&mut buffer)?;

    let blob_header = match BlobHeader::parse_from_bytes(&buffer) {
        Ok(blob) => { blob },
        Err(err) => { return Err(Box::new(err)); }
    };

    let Some(type_) = blob_header.type_ else {
        return Err(Box::new(Error::BlobHeaderTypeMissing(blob_num)));
    };

    let type_ = match type_.as_str() {
        "OSMHeader" => { BlobType::OsmHeader }
        "OSMData"   => { BlobType::OsmData }
        _ => { return Err(Box::new(Error::UnknownBlobHeaderType(type_))); }
    };

    if blob_header.indexdata.is_some() {
        return Err(Box::new(Error::UnknownIndexDataInBlobHeader(blob_num)));
    }

    let Some(data_size) = blob_header.datasize else {
        return Err(Box::new(Error::BlobError(BlobError::NoDataSize, blob_num)));
    };

    let Ok(data_size) = data_size.try_into() else {
        return Err(Box::new(Error::BlobError(BlobError::DataSizeOutOfRange(data_size), blob_num)));
    };

    Ok(BlobInfo {
        data_size,
        type_
    })
}


fn read_blob_data<R: std::io::Read>(
    reader: &mut R,
    size:   usize
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let mut buffer = vec![0u8; size];
    reader.read_exact(&mut buffer)?;

    let blob = match Blob::parse_from_bytes(&buffer) {
        Ok(blob) => { blob },
        Err(err) => { println!("{err}"); return Err(Box::new(err)); }
    };

    match decompress_blob_data(blob) {
        Ok(data) => { Ok(data) },
        Err(err) => { Err(Box::new(err)) }
    }
}


fn decompress_blob_data(blob: Blob) -> Result<Vec<u8>, BlobError>{

    let Some(raw_size) = blob.raw_size else {
        return Err(BlobError::NoRawDataSize);
    };

    let Ok(raw_size) = raw_size.try_into() else {
        return Err(BlobError::DataSizeOutOfRange(raw_size));
    };

    let Some(data) = blob.data else { return Err(BlobError::NoData); };

    match data {
        blob::Data::Raw(_) => {
            Err(BlobError::CompressionNotSupported("Raw"))
        }
        blob::Data::ZlibData(data) => {
            uncompress_zlib(&data, raw_size)
        }
        blob::Data::LzmaData(_) => {
            Err(BlobError::CompressionNotSupported("Lzma"))
        }
        blob::Data::OBSOLETEBzip2Data(_) => {
            Err(BlobError::CompressionNotSupported("Bzip2 (obsolete)"))
        }
        blob::Data::Lz4Data(_) => {
            Err(BlobError::CompressionNotSupported("Lz4"))
        }
        blob::Data::ZstdData(_) => {
            Err(BlobError::CompressionNotSupported("Zstd"))
        }
    }
}


fn uncompress_zlib(compressed_data: &[u8], raw_size: usize) -> Result<Vec<u8>, BlobError> {

    let mut data = vec![0u8; raw_size];
    let (decompressed, rc) =
        zlib_rs::decompress_slice(
            &mut data,
            compressed_data,
            zlib_rs::InflateConfig::default()
        );

    if rc != zlib_rs::ReturnCode::Ok {
        return Err(BlobError::ZlibDecompressionError(
            zlib_return_code_to_string(rc)
        ));
    }

    if decompressed.len() != raw_size {
        return Err(BlobError::DecompressedDataSizeMismatch);
    }

    Ok(data)
}


fn zlib_return_code_to_string(code: zlib_rs::ReturnCode) -> &'static str {

    use zlib_rs::ReturnCode;

    match code {
        ReturnCode::Ok           => "Ok",
        ReturnCode::StreamEnd    => "Stream end",
        ReturnCode::NeedDict     => "Need dict",
        ReturnCode::ErrNo        => "Err No",
        ReturnCode::StreamError  => "Stream Error",
        ReturnCode::DataError    => "Data Error",
        ReturnCode::MemError     => "Mem Error",
        ReturnCode::BufError     => "Buf Error",
        ReturnCode::VersionError => "Version Error"
    }
}


fn parse_osm_header(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

    use protos::osmformat::HeaderBlock;

    let header = match HeaderBlock::parse_from_bytes(data) {
        Ok(header) => { header },
        Err(err)   => { return Err(Box::new(err)); }
    };

    println!("Required features:");
    for feature in header.required_features {
        println!("{feature}");
    }

    println!("Optional features:");
    for feature in header.optional_features {
        println!("{feature}");
    }

    Ok(())
}


#[must_use]
fn create_map(blocks: &[BlockData]) -> MapData {

    let mut map = MapData::create_empty_map();

    for block in blocks {

        for node in &block.nodes {

            map.nodes.insert(node.borrow().id, node.clone());
        }

        for way in &block.ways {

            map.ways.insert(way.borrow().id, way.clone());
        }

        for relation in &block.relations {

            map.relations.insert(relation.borrow().id, relation.clone());
        }
    }

    parser::construct_ways(&mut map);
    parser::construct_relations(&mut map);

    map
}
