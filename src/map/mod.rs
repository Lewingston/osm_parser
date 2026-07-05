
use std::collections::HashMap;

mod feature;

pub use feature::Feature;
pub use feature::FeatureSubType;

pub struct MapData {

    pub nodes:     HashMap<u64, Node>,
    pub ways:      HashMap<u64, Way>,
    pub relations: HashMap<u64, Relation>
}

pub struct Node {

    pub id: u64,
    pub latitude: f64,
    pub longitude: f64,
    pub tags: Option<Tags>
}

pub struct Way {

    pub id: u64,
    pub tags: Option<Tags>
}

pub struct Relation {

    pub id: u64,
    pub tags: Option<Tags>
}

pub struct Tags {

    pub features: Vec<Feature>
}
