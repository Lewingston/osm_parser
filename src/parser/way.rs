
use osm_parser::map::Way;

use serde_json::Value;

use crate::parser::tags;

type JsonObj = serde_json::Map<String, Value>;

pub fn parse(way: &JsonObj) -> Option<Way> {

    let Some(id) = way.get("id").and_then(Value::as_u64) else {
        println!("Way has no id!");
        return None
    };

    let tags = way.get("tags").and_then(tags::parse);

    Some(Way {
        id,
        tags
    })
}
