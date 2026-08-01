
pub mod json;
pub mod pbf;

use crate::map::{
    Id,
    MapData,
    Node,
    Way,
    Relation
};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


pub fn construct_ways(map: &mut MapData) {

    for way in &mut map.ways.values_mut() {

        construct_way(way, &mut map.nodes);
    }
}


fn construct_way(way: &Rc<RefCell<Way>>, nodes: &mut HashMap<Id, Rc<RefCell<Node>>>) {

    let parent_id = way.borrow().id;

    for way_node in &mut way.borrow_mut().child_nodes {

        let child_id = way_node.id;

        let Some(node) = nodes.get(&child_id) else { continue; };
        node.borrow_mut().parent_ways.insert(parent_id, way.clone());
        way_node.node = Some(node.clone());
    }
}


pub fn construct_relations(map: &mut MapData) {

    let mut list_of_nested_relations: Vec<Rc<RefCell<Relation>>> = vec![];

    for relation in &mut map.relations.values_mut() {

        construct_relation(
            relation,
            &mut map.nodes,
            &mut map.ways,
        );

        if !relation.borrow().members.relations.is_empty() {
            list_of_nested_relations.push(relation.clone());
        }
    }

    for relation in &mut list_of_nested_relations {

        construct_nested_relation(relation, &mut map.relations);
    }
}


fn construct_relation(
    relation:  &Rc<RefCell<Relation>>,
    nodes:     &mut HashMap<Id, Rc<RefCell<Node>>>,
    ways:      &mut HashMap<Id, Rc<RefCell<Way>>>)
{
    let parent_id = relation.borrow().id;

    for relation_node in &mut relation.borrow_mut().members.nodes {

        let child_id  = relation_node.id;
        let Some(node) = nodes.get(&child_id) else { continue; };
        node.borrow_mut().parent_relations.insert(parent_id, relation.clone());
        relation_node.node = Some(node.clone());
    }

    for relation_way in &mut relation.borrow_mut().members.ways {

        let child_id = relation_way.id;
        let Some(way) = ways.get(&child_id) else { continue; };
        way.borrow_mut().parent_relations.insert(parent_id, relation.clone());
        relation_way.way = Some(way.clone());
    }
}


fn construct_nested_relation(
    relation:  &Rc<RefCell<Relation>>,
    relations: &mut HashMap<Id, Rc<RefCell<Relation>>>)
{
    let parent_id = relation.borrow().id;

    for relation_relation in &mut relation.borrow_mut().members.relations {

        let child_id = relation_relation.id;
        let Some(child_relation) = relations.get(&child_id) else { continue; };
        child_relation.borrow_mut().parent_relations.insert(parent_id, relation.clone());
        relation_relation.relation = Some(child_relation.clone());
    }
}
