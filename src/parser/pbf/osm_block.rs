
use crate::parser::pbf::protos;
use crate::parser::pbf::protos::{
    osmformat::PrimitiveBlock
};
use protobuf::Message;

use std::rc::Rc;
use std::cell::RefCell;

use crate::map::{
    Node,
    Way,
    Relation
};

use crate::parser::pbf::error::OsmBlockError;
use crate::parser::pbf::string_table::StringTable;

pub type Nodes     = Vec<Rc<RefCell<Node>>>;
pub type Ways      = Vec<Rc<RefCell<Way>>>;
pub type Relations = Vec<Rc<RefCell<Relation>>>;


struct MapData {

    pub nodes:     Nodes,
    pub ways:      Ways,
    pub relations: Relations
}


impl MapData {

    fn new() -> Self {

        Self {
            nodes:     Nodes::new(),
            ways:      Ways::new(),
            relations: Relations::new()
        }
    }
}


pub fn parse(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

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
        println!("Granularity: {gran}");
    }
    if let Some(offset) = block.lat_offset {
        println!("Offset latitude {offset}");
    }
    if let Some(offset) = block.lon_offset {
        println!("Offset longitude {offset}");
    }
    if let Some(date_gran) = block.date_granularity {
        println!("Date granularity: {date_gran}");
    }

    block.parse()?;

    Ok(())
}


trait MapDataParser {

    fn parse(&self, string_table: &StringTable) -> Result<MapData, OsmBlockError>;
}


trait PrimitiveBlockEx {

    fn parse(&self) -> Result<MapData, OsmBlockError>;
}


impl PrimitiveBlockEx for PrimitiveBlock {

    fn parse(&self) -> Result<MapData, OsmBlockError> {

        const THIS_TYPE: &str = "PrimitiveBlock";

        let string_table = StringTable::new(&self.stringtable)?;
        println!("String table entries: {}", string_table.len());

        if self.granularity.is_some() {
            return Err(OsmBlockError::UnsupportedAttribute("granularity", THIS_TYPE));
        }
        if self.lat_offset.is_some() {
            return Err(OsmBlockError::UnsupportedAttribute("lat_offset", THIS_TYPE));
        }
        if self.lon_offset.is_some() {
            return Err(OsmBlockError::UnsupportedAttribute("lon_offset", THIS_TYPE));
        }
        if self.date_granularity.is_some() {
            return Err(OsmBlockError::UnsupportedAttribute("date_granularity", THIS_TYPE));
        }

        for group in &self.primitivegroup {

            group.parse(&string_table)?;
        }

        Ok(MapData::new())
    }
}


impl MapDataParser for protos::osmformat::PrimitiveGroup {

    fn parse(&self, string_table: &StringTable) -> Result<MapData, OsmBlockError> {

        for node in &self.nodes {

            node.parse(string_table)?;
        }

        if let Some(dense_nodes) = &self.dense.0 {

            dense_nodes.parse(string_table)?;
        }

        for way in &self.ways {

            way.parse(string_table)?;
        }

        for relation in &self.relations {

            relation.parse(string_table)?;
        }

        if self.changesets.len() > 0 {
            return Err(OsmBlockError::UnsupportedAttribute("changesets", "PrimitiveGroup"));
        }

        Ok(MapData::new())
    }
}


impl MapDataParser for protos::osmformat::Node {

     fn parse(&self, _string_table: &StringTable) -> Result<MapData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Node"))
     }
}


impl MapDataParser for protos::osmformat::DenseNodes {

    fn parse(&self, string_table: &StringTable) -> Result<MapData, OsmBlockError> {

        if self.id.len() != self.lat.len() ||
           self.id.len() != self.lon.len() {

            return Err(OsmBlockError::WrongNumberOfAttributes(
                format!("Id: {} - Lat: {} - Lon: {}", self.id.len(), self.lat.len(), self.lon.len())
            ));
        }

        println!("Ids: {}", self.id.len());
        println!("lat: {}", self.lat.len());
        println!("lon: {}", self.lon.len());
        println!("keys_vals: {}", self.keys_vals.len());

        let mut keys_vals = self.keys_vals.split(|&i| i == 0);

        //for index in 0..self.id.len() {
        for index in 0..32 {

            //println!("Key vals: {:#?}", keys_vals.next());

            let tags = string_table.get_dense_node_tags(keys_vals.next().unwrap());

            println!("Tags: {:#?}", tags);
        }

        let zero_count = self.keys_vals.iter().
            filter(|i| **i == 0).count();

        /*
        println!("Zero count: {zero_count}");

        println!("{:#?}", &self.keys_vals[0..32]);
        println!("...\n...\n...");
        println!("{:#?}", &self.keys_vals[self.keys_vals.len() - 32..self.keys_vals.len()]);
        */

        /* TODO
        if self.denseinfo.is_some() {
            return Err(OsmBlockError::ParserNotImplemented("denseinfo"));
        }
        */

        Ok(MapData::new())
    }
}


impl MapDataParser for protos::osmformat::Way {

    fn parse(&self, _string_table: &StringTable) -> Result<MapData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Way"))
    }
}


impl MapDataParser for protos::osmformat::Relation {

    fn parse(&self, _string_table: &StringTable) -> Result<MapData, OsmBlockError> {

        Err(OsmBlockError::ParserNotImplemented("Relation"))
    }
}
