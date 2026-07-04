
use std::collections::HashMap;

pub struct Node {

    pub id: u64,
    pub latitude: f64,
    pub longitude: f64
}

pub struct Way {

    pub id: u64
}

pub struct Relation {

    pub id: u64
}

pub struct MapData {

    pub nodes:     HashMap<u64, Node>,
    pub ways:      HashMap<u64, Way>,
    pub relations: HashMap<u64, Relation>
}
