
mod node;
mod way;
mod relation;
mod tags;

use crate::map::{
    Id,
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

    let mut data = MapData::create_empty_map();

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


fn construct_way(way: &Rc<RefCell<Way>>, nodes: &mut HashMap<Id, Rc<RefCell<Node>>>) {

    let parent_id = way.borrow().id;

    for way_node in &mut way.borrow_mut().child_nodes {

        let child_id = way_node.id;

        let Some(node) = nodes.get(&child_id) else { continue; };
        node.borrow_mut().parent_ways.insert(parent_id, way.clone());
        way_node.node = Some(node.clone());
    }
}


fn construct_relations(map: &mut MapData) {

    let mut list_of_nested_relations: Vec<Rc<RefCell<Relation>>> = vec![];

    for relation in &mut map.relations.values_mut() {

        construct_relation(
            relation,
            &mut map.nodes,
            &mut map.ways,
        );

        if !relation.borrow().members.relations.is_empty() {
            list_of_nested_relations.push(relation.clone());
        }
    }

    for relation in &mut list_of_nested_relations {

        construct_nested_relation(relation, &mut map.relations);
    }
}


fn construct_relation(
    relation:  &Rc<RefCell<Relation>>,
    nodes:     &mut HashMap<Id, Rc<RefCell<Node>>>,
    ways:      &mut HashMap<Id, Rc<RefCell<Way>>>)
{
    let parent_id = relation.borrow().id;

    for relation_node in &mut relation.borrow_mut().members.nodes {

        let child_id  = relation_node.id;
        let Some(node) = nodes.get(&child_id) else { continue; };
        node.borrow_mut().parent_relations.insert(parent_id, relation.clone());
        relation_node.node = Some(node.clone());
    }

    for relation_way in &mut relation.borrow_mut().members.ways {

        let child_id = relation_way.id;
        let Some(way) = ways.get(&child_id) else { continue; };
        way.borrow_mut().parent_relations.insert(parent_id, relation.clone());
        relation_way.way = Some(way.clone());
    }
}


fn construct_nested_relation(
    relation:  &Rc<RefCell<Relation>>,
    relations: &mut HashMap<Id, Rc<RefCell<Relation>>>)
{
    let parent_id = relation.borrow().id;

    for relation_relation in &mut relation.borrow_mut().members.relations {

        let child_id = relation_relation.id;
        let Some(child_relation) = relations.get(&child_id) else { continue; };
        child_relation.borrow_mut().parent_relations.insert(parent_id, relation.clone());
        relation_relation.relation = Some(child_relation.clone());
    }
}
