
use crate::map::{
    Way,
    Relation,
    WayNode
};

use serde_json::Value;

use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::tags;

type JsonObj = serde_json::Map<String, Value>;
type JsonArray = Vec<Value>;

pub fn parse(way: &JsonObj) -> Option<Way> {

    let Some(id) = way.get("id").and_then(Value::as_u64) else {
        println!("Way has no id!");
        return None
    };

    let tags = way.get("tags").and_then(tags::parse);

    let nodes = way.get("nodes")
        .and_then(Value::as_array)
        .map_or(Vec::<WayNode>::new(), parse_nodes);

    Some(Way {
        id,
        tags,
        nodes,
        relations: Vec::<Rc<RefCell<Relation>>>::new()
    })
}


fn parse_nodes(nodes: &JsonArray) -> Vec<WayNode> {
    nodes.iter()
        .filter_map(|id| id.as_u64().map(|id| WayNode{id, node: None}))
        .collect()
}

