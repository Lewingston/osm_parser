
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

    _ = block.parse();

    Ok(())
}


trait MapDataParser {

    fn parse(&self) -> Result<MapData, OsmBlockError>;
}


impl MapDataParser for PrimitiveBlock {

    fn parse(&self) -> Result<MapData, OsmBlockError> {

        Ok(MapData::new())
    }
}


impl MapDataParser for protos::osmformat::Node {

     fn parse(&self) -> Result<MapData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Node"))
     }
}


impl MapDataParser for protos::osmformat::DenseNodes {

    fn parse(&self) -> Result<MapData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("DenseNode"))
    }
}


impl MapDataParser for protos::osmformat::Way {

    fn parse(&self) -> Result<MapData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Way"))
    }
}


impl MapDataParser for protos::osmformat::Relation {

    fn parse(&self) -> Result<MapData, OsmBlockError> {

        Err(OsmBlockError::ParserNotImplemented("Relation"))
    }
}
