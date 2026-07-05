
use std::collections::HashMap;
use std::hash::Hash;

use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::Display;


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

#[derive(Display, EnumIter, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Feature {
    Advertising,
    Aerialway,
    Aeroway,
    Amenity,
    Barrier,
    Boundary,
    Building,
    Club,
    Craft,
    #[strum(serialize = "depatures_board")]
    DepaturesBoard,
    Education,
    Emergency,
    Geological,
    Healthcare,
    Highway,
    History,
    Landcover,
    Landuse,
    Leisure,
    #[strum(serialize = "man_made")]
    ManMade,
    Military,
    Natural,
    Office,
    #[strum(serialize = "piste:type")]
    PisteType,
    Place,
    Power,
    #[strum(serialize = "public_transport")]
    PublicTransport,
    Railway,
    Route,
    Shop,
    Telecom,
    Tourism,
    Waterway
}
