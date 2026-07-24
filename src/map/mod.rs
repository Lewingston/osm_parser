
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

mod feature;

pub use feature::Feature;
pub use feature::FeatureSubType;


pub struct MapData {

    pub nodes:     HashMap<u64, Rc<RefCell<Node>>>,
    pub ways:      HashMap<u64, Rc<RefCell<Way>>>,
    pub relations: HashMap<u64, Rc<RefCell<Relation>>>
}


pub struct Node {

    pub id: u64,
    pub latitude: f64,
    pub longitude: f64,
    pub tags: Option<Tags>,

    pub ways:      Vec<Rc<RefCell<Way>>>,
    pub relations: Vec<Rc<RefCell<Relation>>>
}


pub struct Way {

    pub id: u64,
    pub tags: Option<Tags>,

    pub nodes:     Vec<WayNode>,
    pub relations: Vec<Rc<RefCell<Relation>>>
}


pub struct WayNode {

    pub id:   u64,
    pub node: Option<Rc<RefCell<Node>>>
}


pub struct Relation {

    pub id:   u64,
    pub tags: Option<Tags>,

    pub members: RelationMembers,

    pub relations: Vec<Rc<RefCell<Relation>>>
}


pub struct Tags {

    pub features: Vec<Feature>
}


pub struct RelationMembers {

    pub nodes:     Vec<RelationNode>,
    pub ways:      Vec<RelationWay>,
    pub relations: Vec<RelationRelation>
}


#[derive(strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationMemberRole {
    #[strum(serialize = "")]
    None,
    AdminCentre,
    Backward,
    Both,
    Cable,
    CampSite,
    DrinkingWater,
    EmergencyAccessPoint,
    Entry,
    Excursion,
    Exit,
    Fork,
    Forward,
    From,
    Guidepost,
    Historic,
    Inner,
    Junction,
    Label,
    Line,
    Link,
    MainStream,
    Map,
    Outer,
    Platform,
    PlatformExitOnly,
    RouteMarker,
    Shelter,
    SideStream,
    Stop,
    StopEntryOnly,
    StopExitOnly,
    Substation,
    #[strum(serialize = "TMC:Point")]
    TmcPoint,
    To,
    Via,
    WaterTap,
    WaterWell,
}


pub struct RelationNode {
    pub node: Option<Rc<RefCell<Node>>>,
    pub id:   u64,
    pub role: RelationMemberRole
}


pub struct RelationWay {
    pub way:  Option<Rc<RefCell<Way>>>,
    pub id:   u64,
    pub role: RelationMemberRole
}


pub struct RelationRelation {
    pub relation: Option<Rc<RefCell<Relation>>>,
    pub id:       u64,
    pub role:     RelationMemberRole
}
