
use osm_parser::map::Relation;

use serde_json::Value;

type JsonObj = serde_json::Map<String, Value>;

pub fn parse(relation: &JsonObj) -> Option<Relation> {

    let Some(id) = relation.get("id").and_then(Value::as_u64) else {
        println!("Relation has no id!");
        return None
    };

    Some(Relation {
        id
    })
}
