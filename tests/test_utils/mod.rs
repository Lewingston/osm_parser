
use osm_parser::map::MapData;
use osm_parser::map::Node;
use osm_parser::map::Way;
use osm_parser::map::Relation;

use std::cell::Ref;

pub trait MapDataTestExtension {

    fn get_node(&self, id: u64) -> Ref<'_, Node>;
    fn get_way(&self, id: u64) -> Ref<'_, Way>;
    fn get_relation(&self, id: u64) -> Ref<'_, Relation>;
}


impl MapDataTestExtension for MapData {

    fn get_node(&self, id: u64) -> Ref<'_, Node> {

        self.nodes.get(&id).expect("no node with id {id} in this map").borrow()
    }

    fn get_way(&self, id: u64) -> Ref<'_, Way> {

        self.ways.get(&id).expect("no way with id {id} in this map").borrow()
    }

    fn get_relation(&self, id: u64) -> Ref<'_, Relation> {

        self.relations.get(&id).expect("no relation with id {id} in this map").borrow()
    }
}


pub trait NodeTestExtension {

    fn get_parent_way(&self, index: usize) -> Ref<'_, Way>;
    fn get_parent_way_by_id(&self, id: u64) -> Ref<'_, Way>;
    fn get_parent_relation(&self, index: usize) -> Ref<'_, Relation>;
}


impl NodeTestExtension for Node {

    fn get_parent_way(&self, index: usize) -> Ref<'_, Way> {

        self.ways[index].borrow()
    }

    fn get_parent_way_by_id(&self, id: u64) -> Ref<'_, Way> {

        let find = self.ways.iter().find(|&way| way.borrow().id == id);

        find.expect("node {self.id} has no parent way with id {id}").borrow()
    }

    fn get_parent_relation(&self, index: usize) -> Ref<'_, Relation> {

        self.relations[index].borrow()
    }
}


pub trait WayTestExtension {

    fn get_node(&self, index: usize) -> Ref<'_, Node>;
}


impl WayTestExtension for Way {

    fn get_node(&self, index: usize) -> Ref<'_, Node> {

        self.nodes[index].node.as_ref().expect("way has no node at position {index}").borrow()
    }
}


pub trait RelationExtension {

    fn get_node(&self, index: usize) -> Ref<'_, Node>;
    fn get_way(&self, index: usize) -> Ref<'_, Way>;
    fn get_relation(&self, index: usize) -> Ref<'_, Relation>;
    fn get_parent_relation(&self, index: usize) -> Ref<'_, Relation>;
}


impl RelationExtension for Relation {

    fn get_node(&self, index: usize) -> Ref<'_, Node> {

        self.members.nodes[index].node.as_ref().expect("relation {self.id} has no node at position {index}").borrow()
    }

    fn get_way(&self, index: usize) -> Ref<'_, Way> {

        self.members.ways[index].way.as_ref().expect("relation {self.id} has no way at position {index}").borrow()
    }

    fn get_relation(&self, index: usize) -> Ref<'_, Relation> {

        self.members.relations[index].relation.as_ref().expect("relation {self.id} has no relation at position {index}").borrow()
    }

    fn get_parent_relation(&self, index: usize) -> Ref<'_, Relation> {

        self.relations[index].borrow()
    }
}
