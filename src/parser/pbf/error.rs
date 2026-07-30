
#[derive(Debug)]
pub enum Error {

    UnknownBlobHeaderType(String),
    BlobHeaderTypeMissing(usize),
    CompressionNotSupported(String),
    UnknownIndexDataInBlobHeader(usize),
    BlobError(BlobError),
    ZlibDecompressionError(String)
}

impl std::error::Error for Error {}


#[derive(Debug)]
pub enum BlobError {
    NoDataSize,
    NoRawDataSize,
    RawDataSizeOutOfRange(i32),
    NoData,
    DecompressedDataSizeMismatch
}

impl std::error::Error for BlobError {}



impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Error::UnknownBlobHeaderType(msg) => {
                write!(f, "Unknown blob header type: {msg}")
            }
            Error::BlobHeaderTypeMissing(blob_num) => {
                write!(f, "Blob header type missing. Blob nr. {blob_num}")
            }
            Error::CompressionNotSupported(msg) => {
                write!(f, "Unsupported data compression: {msg}")
            },
            Error::UnknownIndexDataInBlobHeader(blob_num) => {
                write!(f, "Unknown index data in blob header. Blob nr. {blob_num}")
            },
            Error::BlobError(blob_error) => {
                blob_error.fmt(f)
            },
            Error::ZlibDecompressionError(msg) => {
                write!(f, "Zlib decompression error: {msg}")
            }
        }
    }
}


impl std::fmt::Display for BlobError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            BlobError::NoDataSize => {
                write!(f, "Blob has no data size")
            }
            BlobError::NoRawDataSize => {
                write!(f, "Blob has no raw data size")
            }
            BlobError::RawDataSizeOutOfRange(size) => {
                write!(f, "Raw data size out of range: {size}")
            }
            BlobError::NoData => {
                write!(f, "Blob has no data")
            }
            BlobError::DecompressedDataSizeMismatch => {
                write!(f, "Unexpected size of decompressed data")
            }
        }
    }
}

