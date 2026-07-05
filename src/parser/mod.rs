
mod node;
mod way;
mod relation;

use osm_parser::map::{
    MapData,
    Node,
    Way,
    Relation
};

use serde_json::{Deserializer, Value};
use std::collections::HashMap;

enum OsmPrimitive {
    Node(Node),
    Way(Way),
    Relation(Relation)
}

pub fn from_file(file_name: &str) -> Result<MapData, Box<dyn std::error::Error>> {

    let file   = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);

    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut data = MapData {
        nodes:     HashMap::<u64, Node>::new(),
        ways:      HashMap::<u64, Way>::new(),
        relations: HashMap::<u64, Relation>::new()
    };

    for value in stream {

        let value = value?;

        let Some(obj)      = value.as_object()   else { continue };
        let Some(elements) = obj.get("elements") else { continue };
        let Some(arr)      = elements.as_array() else { continue };

        for element in arr {

            let Some(result) = parse_element(element) else { continue };
            match result {
                OsmPrimitive::Node    (node)     => { data.nodes    .insert(node.id,     node    ); },
                OsmPrimitive::Way     (way)      => { data.ways     .insert(way.id,      way     ); },
                OsmPrimitive::Relation(relation) => { data.relations.insert(relation.id, relation); }
            }
        }
    }

    Ok(data)
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
