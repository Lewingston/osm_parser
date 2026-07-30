
use crate::map::{
    MapData
};

mod error;

use error::Error;
use error::BlobError;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/osmpbf/mod.rs"));
}

use protos::fileformat::BlobHeader;
use protos::fileformat::Blob;
use protos::fileformat::blob;
use protobuf::Message;


pub fn from_file(file_name: &str) -> Result<MapData, Box<dyn std::error::Error>> {

    let file   = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);

    parse(reader)
}



fn parse<R: std::io::Read>(mut reader: R) -> Result<MapData, Box<dyn std::error::Error>> {

    let map = MapData::create_empty_map();

    let mut blob_count = 0;

    while true {

        let mut buffer = [0; 4];
        match reader.read_exact(&mut buffer) {
            Ok(()) => {

                let header_size = u32::from_be_bytes(buffer) as usize;
                println!("Blob: {blob_count}");
                read_blob(&mut reader, header_size, blob_count)?;
                blob_count += 1;
                println!("");

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

    let blob_size = read_blob_header(reader, header_size, blob_num)?;

    read_blob_data(reader, blob_size as usize)?;

    Ok(())
}


fn read_blob_header<R: std::io::Read>(
    reader:   &mut R,
    size:     usize,
    blob_num: usize
) -> Result<i32, Box<dyn std::error::Error>> {

    let mut buffer = vec![0u8; size];
    reader.read_exact(&mut buffer)?;

    let blob_header = match BlobHeader::parse_from_bytes(&buffer) {
        Ok(blob) => { blob },
        Err(err) => { println!("{err}"); return Err(Box::new(err)); }
    };

    if let Some(type_) = blob_header.type_ {
        match type_.as_str() {
            "OSMHeader" => {}
            "OSMData"   => {}
            _ => { return Err(Box::new(Error::UnknownBlobHeaderType(type_))); }
        }
    } else {
        return Err(Box::new(Error::BlobHeaderTypeMissing(blob_num)));
    }

    if blob_header.indexdata.is_some() {
        return Err(Box::new(Error::UnknownIndexDataInBlobHeader(blob_num)));
    }

    if let Some(datasize) = blob_header.datasize {
        println!("Data size: {datasize}");
        Ok(datasize)
    } else {
        println!("No data size!");
        Ok(0)
    }
}


fn read_blob_data<R: std::io::Read>(
    reader: &mut R,
    size:   usize
) -> Result<(), Box<dyn std::error::Error>> {

    let mut buffer = vec![0u8; size];
    reader.read_exact(&mut buffer)?;

    let blob = match Blob::parse_from_bytes(&buffer) {
        Ok(blob) => { blob },
        Err(err) => { println!("{err}"); return Err(Box::new(err)); }
    };

    parse_blob(blob)?;

    Ok(())
}


fn parse_blob(blob: Blob) -> Result<(), Error>{

    let Some(raw_size) = blob.raw_size else {
        return Err(Error::BlobError(BlobError::NoRawDataSize));
    };

    let Ok(raw_size) = raw_size.try_into() else {
        return Err(Error::BlobError(BlobError::RawDataSizeOutOfRange(raw_size)));
    };

    if let Some(data) = blob.data {

        match data {
            blob::Data::Raw(_) => {
                Err(Error::CompressionNotSupported("Raw".to_string()))
            }
            blob::Data::ZlibData(data) => {
                uncompress_zlib(&data, raw_size)?;
                Ok(())
            }
            blob::Data::LzmaData(_) => {
                Err(Error::CompressionNotSupported("Lzma".to_string()))
            }
            blob::Data::OBSOLETEBzip2Data(_) => {
                Err(Error::CompressionNotSupported("Bzip2 (obsolete)".to_string()))
            }
            blob::Data::Lz4Data(_) => {
                Err(Error::CompressionNotSupported("Lz4".to_string()))
            }
            blob::Data::ZstdData(_) => {
                Err(Error::CompressionNotSupported("Zstd".to_string()))
            }
        }

    } else {
        Err(Error::BlobError(BlobError::NoData))
    }
}


fn uncompress_zlib(compressed_data: &[u8], raw_size: usize) -> Result<(), Error> {

    let mut data = vec![0u8; raw_size];
    let (decompressed, rc) =
        zlib_rs::decompress_slice(
            &mut data,
            compressed_data,
            zlib_rs::InflateConfig::default()
        );

    if rc != zlib_rs::ReturnCode::Ok {
        return Err(Error::ZlibDecompressionError(
            zlib_return_code_to_string(rc)
        ));
    }

    if decompressed.len() != raw_size {
        return Err(Error::BlobError(BlobError::DecompressedDataSizeMismatch));
    }

    Ok(())
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
