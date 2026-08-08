
mod node;
mod way;
mod relation;
mod tags;

use crate::map::{
    MapData,
    Node,
    Way,
    Relation
};

use serde_json as json;
use std::rc::Rc;
use std::cell::RefCell;

use std::path::Path;

enum OsmPrimitive {
    Node(Node),
    Way(Way),
    Relation(Relation)
}

use crate::parser;


/// # Errors
///
/// Will return an error if parsing JSON failed.
pub fn from_string(str: &str) -> Result<MapData, Box<dyn std::error::Error>> {

    let cursor = std::io::Cursor::new(str.as_bytes());
    let reader = std::io::BufReader::new(cursor);

    parse(reader)
}


/// # Errors
///
/// Will return an error if parsing JSON file failed.
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<MapData, Box<dyn std::error::Error>> {

    let file   = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    parse(reader)
}


fn parse<R: std::io::Read>(reader: R) -> Result<MapData, Box<dyn std::error::Error>> {

    let stream = json::Deserializer::from_reader(reader).into_iter::<json::Value>();

    let mut map = MapData::create_empty_map();

    for value in stream {

        let value = value?;

        let Some(obj)      = value.as_object()   else { continue };
        let Some(elements) = obj.get("elements") else { continue };
        let Some(arr)      = elements.as_array() else { continue };

        for element in arr {

            let Some(result) = parse_element(element) else { continue };
            match result {
                OsmPrimitive::Node(node) => {
                    map.nodes.insert(node.id, Rc::new(RefCell::new(node)));
                },
                OsmPrimitive::Way(way) => {
                    map.ways.insert(way.id, Rc::new(RefCell::new(way)));
                },
                OsmPrimitive::Relation(relation) => {
                    map.relations.insert(relation.id, Rc::new(RefCell::new(relation)));
                }
            }
        }
    }

    parser::construct_ways(&mut map);
    parser::construct_relations(&mut map);

    Ok(map)
}


fn parse_element(element: &serde_json::Value) -> Option<OsmPrimitive> {

    let Some(obj) = element.as_object() else {
        println!("Element is not an JSON object!");
        return None
    };

    let Some(element_type) = obj.get("type").and_then(|t| t.as_str()) else {
        println!("Element has no type attribute!");
        return None
    };

    match element_type {
        "node"     => node    ::parse(obj).map(OsmPrimitive::Node),
        "way"      => way     ::parse(obj).map(OsmPrimitive::Way),
        "relation" => relation::parse(obj).map(OsmPrimitive::Relation),
        _ => {
            println!("Element of unknown type: {element_type}");
            None
        }
    }
}


