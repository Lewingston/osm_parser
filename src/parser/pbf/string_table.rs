
use crate::parser::pbf::protos;

use crate::parser::pbf::error::OsmBlockError;

use crate::map;


pub struct StringTable<'table> {

    table: &'table protos::osmformat::StringTable
}


pub enum KeyValueResult {
    Feature(map::Feature),
    Tag(map::Tag)
}


impl<'table> StringTable<'table> {


    pub fn new(table: &'table protos::osmformat::StringTable) -> Self {

        Self {
            table
        }
    }


    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'table str> {

        if index >= self.table.s.len() {
            return None;
        }

        match std::str::from_utf8(&self.table.s[index]) {
            Ok(result) => { Some(result) }
            Err(err)   => {
                println!("Error getting string from string table. Index: {index} - Error: {err}");
                None
            }
        }
    }


    pub fn get_tag_or_feature(
        &self,
        key_index: usize,
        val_index: usize)
    -> Result<KeyValueResult, OsmBlockError> {

        let Some(key) = self.get(key_index) else {
            return Err(OsmBlockError::StringTableAccess(key_index));
        };
        let Some(value) = self.get(val_index) else {
            return Err(OsmBlockError::StringTableAccess(val_index));
        };

        match map::Feature::create(key, value) {
            Some(feature) => { Ok(KeyValueResult::Feature(feature)) }
            None => {
                Ok(KeyValueResult::Tag(
                    map::Tag {
                        key:   key.to_string(),
                        value: value.to_string()
                    }
                ))
            }
        }
    }


    pub fn get_dense_node_tags(
        &self,
        keys_and_values: &[i32]
    ) -> Result<map::Tags, OsmBlockError> {

        let mut features   = Vec::<map::Feature>::new();
        let mut other_tags = Vec::<map::Tag>::new();

        if !keys_and_values.len().is_multiple_of(2) {
            return Err(OsmBlockError::InvalidNumberOfDenseNodeStringTableIndexes(keys_and_values.len()))
        }

        for key_value in keys_and_values.chunks(2) {

            let Ok(key_index) = key_value[0].try_into() else {
                return Err(OsmBlockError::InvalidStringTableIndex(key_value[0]));
            };

            let Ok(val_index) = key_value[1].try_into() else {
                return Err(OsmBlockError::InvalidStringTableIndex(key_value[1]));
            };

            match self.get_tag_or_feature(key_index, val_index)? {
                KeyValueResult::Feature(feature) => {
                    features.push(feature);
                }
                KeyValueResult::Tag(tag) => {
                    other_tags.push(tag);
                }
            }
        }

        Ok(map::Tags {
            features,
            other_tags
        })
    }


    pub fn get_tags(
        &self,
        keys: &[u32],
        vals: &[u32]
    ) -> Result<map::Tags, OsmBlockError> {

        if keys.len() != vals.len() {
            return Err(OsmBlockError::NumberOfKeysAndValsMismatched(keys.len(), vals.len()));
        }

        let mut features   = Vec::<map::Feature>::new();
        let mut other_tags = Vec::<map::Tag>::new();

        for (key_index, val_index) in keys.iter().zip(vals.iter()) {

            match self.get_tag_or_feature((*key_index) as usize, (*val_index) as usize)? {
                KeyValueResult::Feature(feature) => {
                    features.push(feature);
                }
                KeyValueResult::Tag(tag) => {
                    other_tags.push(tag);
                }
            }
        }

        Ok(map::Tags {
            features,
            other_tags
        })
    }
}
