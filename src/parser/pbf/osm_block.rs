
use crate::parser::pbf::protos;
use crate::parser::pbf::protos::{
    osmformat::PrimitiveBlock
};
use protobuf::Message;

use std::rc::Rc;
use std::cell::RefCell;

use crate::map::{
    Id,
    Node,
    Way,
    Relation
};

use crate::parser::pbf::error::OsmBlockError;
use crate::parser::pbf::string_table::StringTable;

pub type Nodes     = Vec<Rc<RefCell<Node>>>;
pub type Ways      = Vec<Rc<RefCell<Way>>>;
pub type Relations = Vec<Rc<RefCell<Relation>>>;


pub struct BlockData {

    pub nodes:     Nodes,
    pub ways:      Ways,
    pub relations: Relations
}


impl BlockData {

    fn new() -> Self {

        Self {
            nodes:     Nodes::new(),
            ways:      Ways::new(),
            relations: Relations::new()
        }
    }
}


struct PrimitiveBlockContext<'block> {

    string_table:      StringTable<'block>,
    granularity:       i32,
    lat_offset:        i64,
    lon_offset:        i64,
    _date_granularity: i32,
}


pub fn parse(data: &[u8]) -> Result<Vec<BlockData>, Box<dyn std::error::Error>> {

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

    Ok(block.parse()?)
}


trait MapDataParser {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError>;
}


trait PrimitiveBlockEx {

    fn parse(&self) -> Result<Vec<BlockData>, OsmBlockError>;
}


impl PrimitiveBlockEx for PrimitiveBlock {

    fn parse(&self) -> Result<Vec<BlockData>, OsmBlockError> {

        let string_table = StringTable::new(&self.stringtable);

        let context = PrimitiveBlockContext {
            string_table,
            granularity:       self.granularity.unwrap_or(100),
            lat_offset:        self.lat_offset.unwrap_or(0),
            lon_offset:        self.lon_offset.unwrap_or(0),
            _date_granularity: self.date_granularity.unwrap_or(1000)
        };

        let mut result = Vec::<BlockData>::with_capacity(self.primitivegroup.len());

        for group in &self.primitivegroup {

            result.append(&mut group.parse(&context)?);
        }

        Ok(result)
    }
}


trait PrimitiveGroupEx {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Vec<BlockData>, OsmBlockError>;
}


impl PrimitiveGroupEx for protos::osmformat::PrimitiveGroup {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Vec<BlockData>, OsmBlockError> {

        let mut result = Vec::<BlockData>::new();

        for node in &self.nodes {

            result.push(node.parse(context)?);
        }

        if let Some(dense_nodes) = &self.dense.0 {

            result.push(dense_nodes.parse(context)?);
        }

        for way in &self.ways {

            result.push(way.parse(context)?);
        }

        for relation in &self.relations {

            result.push(relation.parse(context)?);
        }

        if !self.changesets.is_empty() {
            return Err(OsmBlockError::UnsupportedAttribute("changesets", "PrimitiveGroup"));
        }

        Ok(result)
    }
}


impl MapDataParser for protos::osmformat::Node {

     fn parse(&self, _context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Node"))
     }
}


impl MapDataParser for protos::osmformat::DenseNodes {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError> {

        if self.id.len() != self.lat.len() ||
           self.id.len() != self.lon.len() {

            return Err(OsmBlockError::WrongNumberOfAttributes(
                format!("Id: {} - Lat: {} - Lon: {}", self.id.len(), self.lat.len(), self.lon.len())
            ));
        }

        let mut map_data = BlockData::new();

        let mut keys_vals = self.keys_vals.split(|&i| i == 0);

        let mut id = Id(0);
        let mut latitude:  f64 = 0.0;
        let mut longitude: f64 = 0.0;

        for index in 0..self.id.len() {

            let Some(keys_vals) = keys_vals.next() else {
                return Err(OsmBlockError::DenseNodeKeysValuesError);
            };

            let tags = context.string_table.get_dense_node_tags(keys_vals)?;

            let Ok(current_id) = self.id[index].try_into() else {
                return Err(OsmBlockError::InvalidOsmId(format!("{}", self.id[index])));
            };

            id = id + current_id;

            let granularity = i64::from(context.granularity);
            let lat = 0.000_000_001 * (context.lat_offset + (granularity * self.lat[index])) as f64;
            let lon = 0.000_000_001 * (context.lon_offset + (granularity * self.lon[index])) as f64;

            latitude  += lat;
            longitude += lon;

            let node = Node {
                id,
                latitude,
                longitude,
                tags: Some(tags),
                ..Default::default()
            };

            map_data.nodes.push(Rc::new(RefCell::new(node)));
        }

        Ok(map_data)
    }
}


impl MapDataParser for protos::osmformat::Way {

    fn parse(&self, _context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Way"))
    }
}


impl MapDataParser for protos::osmformat::Relation {

    fn parse(&self, _context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError> {

        Err(OsmBlockError::ParserNotImplemented("Relation"))
    }
}
