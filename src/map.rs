
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::Display;

use enum_dispatch::enum_dispatch;


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

#[enum_dispatch]
pub trait FeatureSubType {

    fn create(&self, attr: &str) -> Option<Feature>;
    fn subtype_to_string(&self) -> String;
}

#[derive(Default, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Office {
    #[default]
    Unknown,
    Accountant,
    Airline,
    Notary,
    Government,
    EmploymentAgency,
    Company,
    Insurance,
    EducationalInstitution,
    TaxAdvisor,
    Lawyer,
    Architect,
    Association,
    Telecommuication,
    EstateAgent,
    Telecommunication,
    It,
    AdvertisingAgency,
    EnergySupplier,
    FinancialAdvisor
}

impl FeatureSubType for Office {

    fn create(&self, attr: &str) -> Option<Feature> {

        match Office::from_str(attr) {
            Ok(sub_type) => Some(Feature::Office(sub_type)),
            Err(_) => None
        }
    }

    fn subtype_to_string(&self) -> String {

        self.to_string()
    }
}

#[derive(Default, Display, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Route {
    #[default]
    Unknown,
    Train,
    Bicycle,
    Bus,
    Boat,
    Ferry,
    Road,
    Tracks,
    Power,
    Hiking,
    Detour,
    Railway,
    Waterway,
    Foot
}

impl FeatureSubType for Route {

    fn create(&self, attr: &str) -> Option<Feature> {

        match Route::from_str(attr) {
            Ok(sub_type) => Some(Feature::Route(sub_type)),
            Err(_) => None
        }
    }

    fn subtype_to_string(&self) -> String {

        self.to_string()
    }
}

#[derive(Display, EnumIter, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[enum_dispatch(FeatureSubType)]
pub enum Feature {
    //Advertising,
    //Aerialway,
    //Aeroway,
    //Amenity,
    //Barrier,
    //Boundary,
    //Building,
    //Club,
    //Craft,
    //#[strum(serialize = "depatures_board")]
    //DepaturesBoard,
    //Education,
    //Emergency,
    //Geological,
    //Healthcare,
    //Highway,
    //History,
    //Landcover,
    //Landuse,
    //Leisure,
    //#[strum(serialize = "man_made")]
    //ManMade,
    //Military,
    //Natural,
    Office(Office),
    //#[strum(serialize = "piste:type")]
    //PisteType,
    //Place,
    //Power,
    //#[strum(serialize = "public_transport")]
    //PublicTransport,
    //Railway,
    Route(Route),
    //Shop,
    //Telecom,
    //Tourism,
    //Waterway
}
