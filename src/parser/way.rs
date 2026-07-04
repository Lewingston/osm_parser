
use osm_parser::map::Way;

use serde_json::Value;

type JsonObj = serde_json::Map<String, Value>;

pub fn parse(way: &JsonObj) -> Option<Way> {

    let Some(id) = way.get("id").and_then(Value::as_u64) else {
        println!("Way has no id!");
        return None
    };

    Some(Way {
        id
    })
}
