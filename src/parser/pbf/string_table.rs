
use crate::parser::pbf::protos;

use crate::parser::pbf::error::OsmBlockError;

use crate::map;


pub struct StringTable<'table> {

    table: &'table protos::osmformat::StringTable
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

            let Ok(value_index) = key_value[1].try_into() else {
                return Err(OsmBlockError::InvalidStringTableIndex(key_value[1]));
            };

            let Some(key) = self.get(key_index) else {
                return Err(OsmBlockError::StringTableAccess(key_index));
            };
            let Some(value) = self.get(value_index) else {
                return Err(OsmBlockError::StringTableAccess(value_index));
            };

            match map::Feature::create(key, value) {
                Some(feature) => { features.push(feature); }
                None => {
                    other_tags.push(map::Tag {
                        key:   key.to_string(),
                        value: value.to_string()
                    });
                }
            }
        }

        Ok(map::Tags {
            features,
            other_tags
        })
    }


    /*
    #[must_use]
    pub fn get_tags(&self) -> Result<map::Tags, OsmBlockError> {

        Ok(map::Tags {
            features: Vec::<map::Feature>::new(),
            other_tags: Vec::<map::Tag>::new()
        })
    }


    #[must_use]
    pub fn len(&self) -> usize {

        self.table.s.len()
    }
    */
}
