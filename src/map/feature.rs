
use std::hash::Hash;
use std::str::FromStr;

use osm_parser_macros::FeatureSubType;
use osm_parser_macros::feature_sub_type;

use strum_macros::{
    EnumIter,
    EnumString,
    Display
};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FeatureSubType {

    fn create(&self, attr: &str) -> Option<Feature>;
    fn subtype_to_string(&self) -> String;
}

#[feature_sub_type]
pub enum Office {
    Accountant,
    AdvertisingAgency,
    Airline,
    Architect,
    Association,
    Company,
    EducationalInstitution,
    EmploymentAgency,
    EnergySupplier,
    EstateAgent,
    FinancialAdvisor,
    Government,
    Insurance,
    It,
    Lawyer,
    Notary,
    TaxAdvisor,
    Telecommuication,
    Telecommunication,
}

#[feature_sub_type]
pub enum Route {
    Bicycle,
    Boat,
    Bus,
    Detour,
    Ferry,
    Foot,
    Hiking,
    Power,
    Railway,
    Road,
    Tracks,
    Train,
    Waterway,
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
