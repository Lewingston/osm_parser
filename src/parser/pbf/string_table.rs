
use crate::parser::pbf::protos;

use crate::parser::pbf::error::OsmBlockError;


pub struct StringTable<'table> {

    table: &'table protos::osmformat::StringTable
}


impl<'table> StringTable<'table> {

    #[must_use]
    pub fn new(table: &'table protos::osmformat::StringTable) -> Result<Self, OsmBlockError> {

        let result = Self {
            table
        };

        /*
        for index in 0..result.len() {
            let Some(string) = result.get(index) else { continue; };
            println!("{string}");
        }
        */

        Ok(result)
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
                return None;
            }
        }
    }

    #[must_use]
    pub fn get_dense_node_tags(&self) {

    }


    #[must_use]
    pub fn get_tags(&self) {

    }


    #[must_use]
    pub fn len(&self) -> usize {

        self.table.s.len()
    }
}
