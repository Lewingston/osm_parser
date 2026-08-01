
use crate::map::{
    Tag,
    Tags,
    Feature
};

use serde_json::Value;


pub fn parse(tags: &Value) -> Option<Tags> {

    let Some(tags) = tags.as_object() else {
        println!("Tags element is not an JSON object!");
        return None
    };

    let mut features   = Vec::<Feature>::new();
    let mut other_tags = Vec::<Tag>::new();

    for tag in tags {

        let key = tag.0;
        let Some(value) = tag.1.as_str() else {
            println!("Map attribute value is not a string: {key} - {}", tag.1);
            continue;
        };

        match Feature::create(key, value) {
            Some(feature) => {
                features.push(feature);
            }
            None => {
                other_tags.push(Tag::new(key.to_string(), value.to_string()));
            }
        }
    }

    Some(Tags {
        features,
        other_tags
    })
}
