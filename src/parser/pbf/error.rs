
#[derive(Debug)]
pub enum Error {

    UnknownBlobHeaderType(String),
    BlobHeaderTypeMissing(usize),
    UnknownIndexDataInBlobHeader(usize),
    #[allow(clippy::enum_variant_names)]
    BlobError(BlobError, usize)
}

impl std::error::Error for Error {}


#[derive(Debug)]
pub enum BlobError {
    NoDataSize,
    NoRawDataSize,
    DataSizeOutOfRange(i32),
    NoData,
    CompressionNotSupported(&'static str),
    DecompressedDataSizeMismatch,
    ZlibDecompressionError(&'static str)
}

impl std::error::Error for BlobError {}


#[derive(Debug)]
pub enum OsmBlockError {
    ParserNotImplemented(&'static str),
    UnsupportedAttribute(&'static str, &'static str),
    WrongNumberOfAttributes(String),
    InvalidStringTableIndex(i32),
    StringTableAccess(usize),
    InvalidNumberOfDenseNodeStringTableIndexes(usize),
    InvalidOsmId(String),
    OsmIdUnderflow(u64, i64),
    DenseNodeKeysValuesError,
    MissingAttribute(&'static str),
    NumberOfKeysAndValsMismatched(usize, usize),
    RelationWithoutMembers(u64),
    UnknownRelationTypeId(i32)
}


impl std::error::Error for OsmBlockError {}


impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Error::UnknownBlobHeaderType(msg) => {
                write!(f, "Unknown blob header type: {msg}")
            }
            Error::BlobHeaderTypeMissing(blob_num) => {
                write!(f, "Blob header type missing. Blob nr. {blob_num}")
            }
            Error::UnknownIndexDataInBlobHeader(blob_num) => {
                write!(f, "Unknown index data in blob header. Blob nr. {blob_num}")
            },
            Error::BlobError(blob_error, blob_num) => {
                write!(f, "Error in blob Nr. {blob_num} - {blob_error}")
            },
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
            BlobError::DataSizeOutOfRange(size) => {
                write!(f, "Data size out of range: {size}")
            }
            BlobError::NoData => {
                write!(f, "Blob has no data")
            }
            BlobError::CompressionNotSupported(msg) => {
                write!(f, "Unsupported data compression: {msg}")
            }
            BlobError::DecompressedDataSizeMismatch => {
                write!(f, "Unexpected size of decompressed data")
            }
            BlobError::ZlibDecompressionError(msg) => {
                write!(f, "Zlib decompression error: {msg}")
            }
        }
    }
}


impl std::fmt::Display for OsmBlockError {

     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

         match self {
            OsmBlockError::ParserNotImplemented(msg) => {
                write!(f, "Parser not implemented for {msg}")
            }
            OsmBlockError::UnsupportedAttribute(attr, data) => {
                write!(f, "Unsupported attribute \"{attr}\" in {data}")
            }
            OsmBlockError::WrongNumberOfAttributes(msg) => {
                write!(f, "Wrong number of attributes: {msg}")
            }
            OsmBlockError::InvalidStringTableIndex(index) => {
                write!(f, "Invalid string table index: {index}")
            }
            OsmBlockError::StringTableAccess(index) => {
                write!(f, "Unable to get string from string table: {index}")
            }
            OsmBlockError::InvalidNumberOfDenseNodeStringTableIndexes(count) => {
                write!(f, "Wrong number of dense node string table indexes: {count}")
            }
            OsmBlockError::InvalidOsmId(id) => {
                write!(f, "Invalid OSM Id: {id}")
            }
            OsmBlockError::OsmIdUnderflow(id1, id2) => {
                write!(f, "Delta codes OSM Id underflow: {id1} - {id2}")
            }
            OsmBlockError::DenseNodeKeysValuesError => {
                write!(f, "Failed to access keys and values of dense node")
            }
            OsmBlockError::MissingAttribute(attr) => {
                write!(f, "Missing attribute: {attr}")
            }
            OsmBlockError::NumberOfKeysAndValsMismatched(key_count, val_count) => {
                write!(f, "Mismatch between number of keys {key_count} and values {val_count}")
            }
            OsmBlockError::RelationWithoutMembers(id) => {
                write!(f, "Relation ({id}) without members")
            }
            OsmBlockError::UnknownRelationTypeId(id) => {
                write!(f, "Unknown relation type id {id}")
            }
         }
     }
}

