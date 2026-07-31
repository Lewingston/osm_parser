
use crate::map::{
    MapData
};

mod error;

use error::Error;
use error::BlobError;

mod osm_block;

#[allow(warnings)]
#[allow(clippy::all)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/osmpbf/mod.rs"));
}

use protos::fileformat::BlobHeader;
use protos::fileformat::Blob;
use protos::fileformat::blob;
use protobuf::Message;


enum BlobType {
    OsmHeader,
    OsmData
}


struct BlobInfo {
    data_size: usize,
    type_:     BlobType
}


/// # Errors
///
/// Returns an error when parsing of the file failed.
pub fn from_file(file_name: &str) -> Result<MapData, Box<dyn std::error::Error>> {

    let file   = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);

    parse(reader)
}



fn parse<R: std::io::Read>(mut reader: R) -> Result<MapData, Box<dyn std::error::Error>> {

    let map = MapData::create_empty_map();

    let mut blob_count = 0;

    //while true {
    for _ in 0..2 {

        let mut buffer = [0; 4];
        match reader.read_exact(&mut buffer) {
            Ok(()) => {

                let header_size = u32::from_be_bytes(buffer) as usize;
                println!("Blob: {blob_count}");
                read_blob(&mut reader, header_size, blob_count)?;
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

    Ok(map)
}


fn read_blob<R: std::io::Read>(
    reader:      &mut R,
    header_size: usize,
    blob_num:    usize
) -> Result<(), Box<dyn std::error::Error>> {

    let blob_info = read_blob_header(reader, header_size, blob_num)?;

    let blob_data = read_blob_data(reader, blob_info.data_size)?;

    match blob_info.type_ {
        BlobType::OsmHeader => { parse_osm_header(&blob_data)?; }
        BlobType::OsmData   => { osm_block::parse(&blob_data)?; }
    }

    Ok(())
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
            Err(BlobError::CompressionNotSupported("Raw".to_string()))
        }
        blob::Data::ZlibData(data) => {
            uncompress_zlib(&data, raw_size)
        }
        blob::Data::LzmaData(_) => {
            Err(BlobError::CompressionNotSupported("Lzma".to_string()))
        }
        blob::Data::OBSOLETEBzip2Data(_) => {
            Err(BlobError::CompressionNotSupported("Bzip2 (obsolete)".to_string()))
        }
        blob::Data::Lz4Data(_) => {
            Err(BlobError::CompressionNotSupported("Lz4".to_string()))
        }
        blob::Data::ZstdData(_) => {
            Err(BlobError::CompressionNotSupported("Zstd".to_string()))
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


fn zlib_return_code_to_string(code: zlib_rs::ReturnCode) ->  String {

    use zlib_rs::ReturnCode;

    match code {
        ReturnCode::Ok           => "Ok".to_string(),
        ReturnCode::StreamEnd    => "Stream end".to_string(),
        ReturnCode::NeedDict     => "Need dict".to_string(),
        ReturnCode::ErrNo        => "Err No".to_string(),
        ReturnCode::StreamError  => "Stream Error".to_string(),
        ReturnCode::DataError    => "Data Error".to_string(),
        ReturnCode::MemError     => "Mem Error".to_string(),
        ReturnCode::BufError     => "Buf Error".to_string(),
        ReturnCode::VersionError => "Version Error".to_string()
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


/*
fn parse_osm_data(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

    use protos::osmformat::PrimitiveBlock;

    let block = match PrimitiveBlock::parse_from_bytes(data) {
        Ok(block) => { block },
        Err(err)  => { return Err(Box::new(err)); }
    };

    println!("String table: {}", block.stringtable.s.len());

    println!("Primitive Groups: {}", block.primitivegroup.len());

    for group in &block.primitivegroup {

        if !group.nodes.is_empty() {
            println!("Nodes: {}", group.nodes.len());
        }

        if group.dense.is_some() {
            println!("Dense node block");
        }

        if !group.ways.is_empty() {
            println!("Ways: {}", group.ways.len());
        }

        if !group.relations.is_empty() {
            println!("Relations: {}", group.relations.len());
        }
    }

    if let Some(gran) = block.granularity {
        println!("Granularity: {}", gran);
    }
    if let Some(offset) = block.lat_offset {
        println!("Offset latitude {}", offset);
    }
    if let Some(offset) = block.lon_offset {
        println!("Offset longitude {}", offset);
    }
    if let Some(date_gran) = block.date_granularity {
        println!("Date granularity: {}", date_gran);
    }

    Ok(())
}
*/
