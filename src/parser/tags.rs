
use osm_parser::map::Tags;
use osm_parser::map::Feature;
use osm_parser::map::FeatureSubType;

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

    let mut features = Vec::<Feature>::new();

    for feature in Feature::iter() {

        let feature_name = feature.to_string();

        let Some(feat_attr) = tags.get(&feature_name) else { continue };
        let Some(feat_attr) = feat_attr.as_str() else {
            println!("Map feature attribute is not a string: {feature}");
            continue
        };

        match feature.create(feat_attr) {
            Some(feature) => features.push(feature),
            None => { println!("{feature_name} - {feat_attr}"); }
        }
    }

    features
}
