
use osm_parser::map::Tags;
use osm_parser::map::Feature;

use serde_json::Value;

use strum::IntoEnumIterator;

type JsonObj = serde_json::Map<String, Value>;


pub fn parse(tags: &Value) -> Option<Tags> {

    let Some(tags) = tags.as_object() else {
        println!("Tags element is not an JSON object!");
        return None
    };

    let features = get_features(tags);

    Some(Tags {
        features
    })
}


fn get_features(tags: &JsonObj) -> Vec<Feature> {

    Feature::iter().
        filter(|feature| tags.get(&feature.to_string()).is_some()).
        collect()
}
