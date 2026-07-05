
use osm_parser::map::Node;

use serde_json::Value;

use crate::parser::tags;

type JsonObj = serde_json::Map<String, Value>;

pub fn parse(node: &JsonObj) -> Option<Node> {

    let Some(id) = node.get("id").and_then(Value::as_u64) else {
        println!("Node has no id!");
        return None
    };

    let Some(latitude) = node.get("lat").and_then(Value::as_f64) else {
        println!("Node has no lat!");
        return None
    };

    let Some(longitude) = node.get("lon").and_then(Value::as_f64) else {
        println!("Node has no lon!");
        return None
    };

    let tags = node.get("tags").and_then(tags::parse);

    Some(Node {
        id,
        latitude,
        longitude,
        tags
    })
}
