
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;

mod feature;

pub use feature::Feature;
pub use feature::FeatureSubType;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(pub u64);


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


pub struct Way {

    pub id:   Id,
    pub tags: Option<Tags>,

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
