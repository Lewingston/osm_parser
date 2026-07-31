
use crate::map::{
    Tag,
    Tags,
    Feature,
    FeatureSubType
};

use serde_json::Value;

use strum::IntoEnumIterator;

type JsonObj = serde_json::Map<String, Value>;


pub fn parse(tags: &Value) -> Option<Tags> {

    let Some(tags) = tags.as_object() else {
        println!("Tags element is not an JSON object!");
        return None
    };

    //let features = get_features(tags);

    let mut features   = Vec::<Feature>::new();
    let mut other_tags = Vec::<Tag>::new();

    for tag in tags {

        let key = tag.0;
        let Some(value) = tag.1.as_str() else {
            println!("Map attribute value is not a string: {key} - {}", tag.1);
            continue;
        };

        if key.parse::<Feature>().is_ok() {

            match create_feature(key, value) {
                Some(feature) => {
                    features.push(feature);
                }
                None => {
                    println!("{key} - {value}");
                    other_tags.push(Tag::new(key.to_string(), value.to_string()));
                }
            };
        } else {

            other_tags.push(Tag::new(key.to_string(), value.to_string()))
        }
    }

    Some(Tags {
        features,
        other_tags
    })
}


fn create_feature(type_: &str, sub_type: &str) -> Option<Feature> {

    for feature in Feature::iter() {

        if type_ == feature.to_string() {
            return feature.create(sub_type);
        }
    }

    None
}
