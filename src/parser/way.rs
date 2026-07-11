
use osm_parser::map::Way;
use osm_parser::map::Node;

use serde_json::Value;

use std::rc::Rc;
use std::collections::HashMap;

use crate::parser::tags;

type JsonObj = serde_json::Map<String, Value>;
type JsonArray = Vec<Value>;

type Nodes = HashMap::<u64, Option<Rc<Node>>>;

pub fn parse(way: &JsonObj) -> Option<Way> {

    let Some(id) = way.get("id").and_then(Value::as_u64) else {
        println!("Way has no id!");
        return None
    };

    let tags = way.get("tags").and_then(tags::parse);

    let nodes = way.get("nodes")
        .and_then(Value::as_array)
        .map_or(Nodes::new(), parse_nodes);

    Some(Way {
        id,
        tags,
        nodes
    })
}


fn parse_nodes(nodes: &JsonArray) -> Nodes {
    nodes.iter()
        .filter_map(|id| id.as_u64().map(|id| (id, None)))
        .collect()
}

