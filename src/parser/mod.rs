
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

use serde_json::{Deserializer, Value};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

enum OsmPrimitive {
    Node(Node),
    Way(Way),
    Relation(Relation)
}


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
pub fn from_file(file_name: &str) -> Result<MapData, Box<dyn std::error::Error>> {

    let file   = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);

    parse(reader)
}


fn parse<R: std::io::Read>(reader: R) -> Result<MapData, Box<dyn std::error::Error>> {

    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut data = MapData {
        nodes:     HashMap::<u64, Rc<RefCell<Node>>>::new(),
        ways:      HashMap::<u64, Rc<RefCell<Way>>>::new(),
        relations: HashMap::<u64, Rc<RefCell<Relation>>>::new()
    };

    for value in stream {

        let value = value?;

        let Some(obj)      = value.as_object()   else { continue };
        let Some(elements) = obj.get("elements") else { continue };
        let Some(arr)      = elements.as_array() else { continue };

        for element in arr {

            let Some(result) = parse_element(element) else { continue };
            match result {
                OsmPrimitive::Node(node) => {
                    data.nodes.insert(node.id, Rc::new(RefCell::new(node)));
                },
                OsmPrimitive::Way(way) => {
                    data.ways.insert(way.id, Rc::new(RefCell::new(way)));
                },
                OsmPrimitive::Relation(relation) => {
                    data.relations.insert(relation.id, Rc::new(RefCell::new(relation)));
                }
            }
        }
    }

    construct_ways(&mut data);
    construct_relations(&mut data);

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


fn construct_ways(map: &mut MapData) {

    for way in &mut map.ways.values_mut() {

        construct_way(way, &mut map.nodes);
    }
}


fn construct_way(way: &Rc<RefCell<Way>>, nodes: &mut HashMap<u64, Rc<RefCell<Node>>>) {

    for way_node in &mut way.borrow_mut().nodes {

        let Some(node) = nodes.get(&way_node.id) else { continue; };
        node.borrow_mut().ways.push(way.clone());
        way_node.node = Some(node.clone());
    }
}


fn construct_relations(map: &mut MapData) {

    for relation in &mut map.relations.values_mut() {

        construct_relation(
            relation,
            &mut map.nodes,
            &mut map.ways,
        );
    }
}


fn construct_relation(
    relation: &Rc<RefCell<Relation>>,
    nodes:    &mut HashMap<u64, Rc<RefCell<Node>>>,
    ways:     &mut HashMap<u64, Rc<RefCell<Way>>>)
{

    for relation_node in &mut relation.borrow_mut().members.nodes {

        let Some(node) = nodes.get(&relation_node.id) else { continue; };
        node.borrow_mut().relations.push(relation.clone());
        relation_node.node = Some(node.clone());
    }

    for relation_way in &mut relation.borrow_mut().members.ways {

        let Some(way) = ways.get(&relation_way.id) else { continue; };
        way.borrow_mut().relations.push(relation.clone());
        relation_way.way = Some(way.clone());
    }

    // TODO: Relations
}
