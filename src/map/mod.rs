
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;

pub mod feature;

pub use feature::Feature;
pub use feature::FeatureSubType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(pub u64);

impl std::ops::Add<i64> for Id {

    type Output = Id;

    fn add(self, rhs: i64) -> Id {

        if rhs < 0 {
            Id(self.0 - rhs.abs() as u64)
        } else {
            Id(self.0 + rhs as u64)
        }
    }
}


pub type NodeMap     = HashMap<Id, Rc<RefCell<Node>>>;
pub type WayMap      = HashMap<Id, Rc<RefCell<Way>>>;
pub type RelationMap = HashMap<Id, Rc<RefCell<Relation>>>;


pub struct MapData {

    pub nodes:     NodeMap,
    pub ways:      WayMap,
    pub relations: RelationMap
}


impl MapData {

    #[must_use]
    pub fn create_empty_map() -> Self {

        Self {
            nodes:     NodeMap::new(),
            ways:      WayMap::new(),
            relations: RelationMap::new()
        }
    }

    #[must_use]
    pub fn get_node(&self, id: Id) -> Option<Ref<'_, Node>> {

        self.nodes.get(&id).map(|node| node.borrow())
    }

    #[must_use]
    pub fn get_way(&self, id: Id) -> Option<Ref<'_, Way>> {

        self.ways.get(&id).map(|way| way.borrow())
    }

    #[must_use]
    pub fn get_relation(&self, id: Id) -> Option<Ref<'_, Relation>> {

        self.relations.get(&id).map(|relation| relation.borrow())
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = Ref<'_, Node>> {

        self.nodes.values().map(|node| node.borrow())
    }


    pub fn iter_ways(&self) -> impl Iterator<Item = Ref<'_, Way>> {

        self.ways.values().map(|way| way.borrow())
    }


    pub fn iter_relations(&self) -> impl Iterator<Item = Ref<'_, Relation>> {

        self.relations.values().map(|relation| relation.borrow())
    }

    #[must_use]
    pub fn get_dimensions(&self) -> Dimensions {

        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;
        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;

        for node in self.nodes.values().map(|node| node.borrow()) {

            if node.latitude < min_lat {
                min_lat = node.latitude;
            }

            if node.latitude > max_lat {
                max_lat = node.latitude;
            }

            if node.longitude < min_lon {
                min_lon = node.longitude;
            }

            if node.longitude > max_lon {
                max_lon = node.longitude;
            }
        }

        Dimensions {
            min_lat,
            min_lon,
            max_lat,
            max_lon
        }
    }
}


pub struct Node {

    pub id:        Id,
    pub latitude:  f64,
    pub longitude: f64,
    pub tags:      Option<Tags>,

    pub parent_ways:      WayMap,
    pub parent_relations: RelationMap
}


impl Node {

    #[must_use]
    pub fn get_parent_way(&self, id: Id) -> Option<Ref<'_, Way>> {

        self.parent_ways.get(&id).map(|way| way.borrow())
    }

    #[must_use]
    pub fn get_parent_relation(&self, id: Id) -> Option<Ref<'_, Relation>> {

        self.parent_relations.get(&id).map(|relation| relation.borrow())
    }
}


impl Default for Node {

    fn default() -> Self {

        Self {
            id:               Id(0),
            latitude:         0.0,
            longitude:        0.0,
            tags:             None,
            parent_ways:      WayMap::new(),
            parent_relations: RelationMap::new()
        }
    }
}


pub struct Way {

    pub id:   Id,
    pub tags: Option<Tags>, // TODO: Do not make tags optional!

    pub child_nodes:      Vec<WayNode>,
    pub parent_relations: RelationMap
}


impl Way {

    #[must_use]
    pub fn get_parent_relation(&self, id: Id) -> Option<Ref<'_, Relation>> {

        self.parent_relations.get(&id).map(|relation| relation.borrow())
    }

    #[must_use]
    pub fn get_child_node(&self, index: usize) -> Option<Ref<'_, Node>> {

        self.child_nodes[index].node.as_ref().map(|way| way.borrow())
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {

        !self.child_nodes.iter().any(|way_node| way_node.node.is_none())
    }
}


pub struct WayNode {

    pub id:   Id,
    pub node: Option<Rc<RefCell<Node>>>
}


pub struct Relation {

    pub id:   Id,
    pub tags: Option<Tags>,

    pub members: RelationMembers,

    pub parent_relations: RelationMap
}


impl Relation
{
    #[must_use]
    pub fn get_parent_relation(&self, id: Id) -> Option<Ref<'_, Relation>> {

        self.parent_relations.get(&id).map(|relation| relation.borrow())
    }

    #[must_use]
    pub fn get_child_node(&self, index: usize) -> Option<Ref<'_, Node>> {

        self.members.nodes[index].node.as_ref().map(|node| node.borrow())
    }

    #[must_use]
    pub fn get_child_way(&self, index: usize) -> Option<Ref<'_, Way>> {

        self.members.ways[index].way.as_ref().map(|way| way.borrow())
    }

    #[must_use]
    pub fn get_child_relation(&self, index: usize) -> Option<Ref<'_, Relation>> {

        self.members.relations[index].relation.as_ref().map(|relation| relation.borrow())
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {

        let has_all_nodes = !self.members.nodes.iter()
            .any(|relation_node| relation_node.node.is_none());

        if !has_all_nodes {
            return false;
        }

        let has_all_ways = !self.members.ways.iter()
            .any(|relation_way| relation_way.way.is_none());

        if !has_all_ways {
            return false;
        }

        let has_all_relations = !self.members.relations.iter()
            .any(|rel_relation| rel_relation.relation.is_none());

        if !has_all_relations {
            return false;
        }

        true
    }
}


#[derive(PartialEq, Debug)]
pub struct Tag {

    pub key:   String,
    pub value: String
}


impl Tag {

    #[must_use]
    pub fn new(key: String, value: String) -> Self {

        Self { key, value }
    }
}


#[derive(Debug)]
pub struct Tags {

    pub features:   Vec<Feature>,
    pub other_tags: Vec<Tag>
}


pub struct RelationMembers {

    pub nodes:     Vec<RelationNode>,
    pub ways:      Vec<RelationWay>,
    pub relations: Vec<RelationRelation>
}


impl RelationMembers {

    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes:     Vec::<RelationNode>::new(),
            ways:      Vec::<RelationWay>::new(),
            relations: Vec::<RelationRelation>::new()
        }
    }
}


#[derive(strum_macros::EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum RelationMemberRole {
    #[strum(serialize = "")]
    None,
    Across,
    AdminCentre,
    Address,
    Alternative,
    Apex,
    ApplyTo,
    Approach,
    Associated,
    Backward,
    Basement,
    Bench,
    Bin,
    Both,
    Building,
    Buildingpart,
    Cable,
    CampSite,
    Child,
    Connection,
    Contains,
    Crossing,
    Detour,
    Device,
    DrinkingWater,
    Edge,
    EmergencyAccessPoint,
    End,
    Entrance,
    Entry,
    Excursion,
    Exit,
    Facility,
    Footprint,
    Force,
    Fork,
    Forward,
    From,
    Guidepost,
    Hip,
    Historic,
    House,
    Inner,
    Junction,
    Label,
    Landuse,
    Lateral,
    Latvia,
    Line,
    Link,
    Lower,
    Main,
    MainStream,
    Map,
    Member,
    MemberState,
    Node,
    Other,
    Outer,
    Outline,
    Part,
    Platform,
    PlatformEntryOnly,
    PlatformExitOnly,
    Ridge,
    Room,
    Route,
    RouteMarker,
    RouteMaster,
    Shell,
    Shelter,
    Sidepath,
    SideStream,
    Sign,
    Signal,
    Spring,
    Start,
    StartingPoint,
    Stop,
    StopPosition,
    StopEntryOnly,
    StopExitOnly,
    Subarea,
    SubStation,
    Street,
    Substation,
    This,
    Tickets,
    #[strum(serialize = "TMC:Point")]
    TmcPoint,
    #[strum(serialize = "TMC:Road")]
    TmcRoad,
    #[strum(serialize = "TMC:Segment")]
    TmcSegment,
    To,
    TrafficSignals,
    TramStop,
    Under,
    Upper,
    Via,
    Ways,
    WaterTap,
    WaterWell,
}


pub struct RelationNode {
    pub node: Option<Rc<RefCell<Node>>>,
    pub id:   Id,
    pub role: RelationMemberRole
}


pub struct RelationWay {
    pub way:  Option<Rc<RefCell<Way>>>,
    pub id:   Id,
    pub role: RelationMemberRole
}


pub struct RelationRelation {
    pub relation: Option<Rc<RefCell<Relation>>>,
    pub id:       Id,
    pub role:     RelationMemberRole
}


#[derive(Clone)]
pub struct Dimensions {

    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64
}


impl Dimensions {

    #[must_use]
    pub fn get_center(&self) -> (f64, f64) {
        (
            self.min_lon + self.get_width() / 2.0,
            self.min_lat + self.get_height() / 2.0
        )
    }

    #[must_use]
    pub fn get_width(&self) -> f64 {

        self.max_lon - self.min_lon
    }

    #[must_use]
    pub fn get_height(&self) -> f64 {

        self.max_lat - self.min_lat
    }


    pub fn expand(&mut self, dim: &Dimensions) {

        self.min_lat = self.min_lat.min(dim.min_lat);
        self.min_lon = self.min_lon.min(dim.min_lon);
        self.max_lat = self.max_lat.max(dim.max_lat);
        self.max_lon = self.max_lon.max(dim.max_lon);
    }
}


impl Default for Dimensions {

    fn default() -> Self {

        Self {
            min_lat: f64::MAX,
            min_lon: f64::MAX,
            max_lat: f64::MIN,
            max_lon: f64::MIN
        }
    }
}
