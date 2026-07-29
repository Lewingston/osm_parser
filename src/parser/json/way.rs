
use crate::map::{
    Id,
    Way,
    RelationMap,
    WayNode
};

use serde_json::Value;

use crate::parser::json::tags;

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
        id: Id(id),
        tags,
        child_nodes: nodes,
        parent_relations: RelationMap::new()
    })
}


fn parse_nodes(nodes: &JsonArray) -> Vec<WayNode> {
    nodes.iter()
        .filter_map(|id| id.as_u64().map(|id| WayNode{id: Id(id), node: None}))
        .collect()
}

