
use osm_parser::parser;

mod test_utils;

use test_utils::MapDataTestExtension;
use test_utils::NodeTestExtension;
use test_utils::WayTestExtension;
use test_utils::RelationExtension;


#[test]
fn test_node_parsing() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 1.0,
                "lon": 3.0
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            let node = map_data.get_node(1);

            assert_eq!(node.id, 1);
            assert_eq!(node.latitude, 1.0);
            assert_eq!(node.longitude, 3.0);
            assert_eq!(node.ways.len(), 0);
            assert_eq!(node.relations.len(), 0);

        },
        Err(_) => {
            assert!(false);
        }
    }
}


#[test]
fn test_way_parsing() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 0.0,
                "lon": 0.0
            },
            {
                "type": "node",
                "id": 2,
                "lat": 1.0,
                "lon": 1.0
            },
            {
                "type": "node",
                "id": 3,
                "lat": 2.0,
                "lon": 2.0
            },
            {
                "type": "way",
                "id": 4,
                "nodes": [
                    3,
                    2,
                    1
                ]
            },
            {
                "type": "way",
                "id": 5,
                "nodes": [
                    1,
                    2
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            assert_eq!(map_data.nodes.len(), 3);
            assert_eq!(map_data.ways.len(), 2);

            let node_a = map_data.get_node(1);
            let node_b = map_data.get_node(2);
            let node_c = map_data.get_node(3);

            let way_a = map_data.get_way(4);
            let way_b = map_data.get_way(5);

            assert_eq!(node_a.ways.len(), 2);
            assert!(std::ptr::eq(&*node_a.get_parent_way_by_id(4), &*way_a));
            assert!(std::ptr::eq(&*node_a.get_parent_way_by_id(5), &*way_b));

            assert_eq!(node_b.ways.len(), 2);
            assert!(std::ptr::eq(&*node_b.get_parent_way_by_id(4), &*way_a));
            assert!(std::ptr::eq(&*node_b.get_parent_way_by_id(5), &*way_b));

            assert_eq!(node_c.ways.len(), 1);
            assert!(std::ptr::eq(&*node_c.get_parent_way(0), &*way_a));

            assert_eq!(way_a.id, 4);
            assert_eq!(way_b.id, 5);

            assert_eq!(way_a.nodes.len(), 3);
            assert!(std::ptr::eq(&*way_a.get_node(0), &*node_c));
            assert!(std::ptr::eq(&*way_a.get_node(1), &*node_b));
            assert!(std::ptr::eq(&*way_a.get_node(2), &*node_a));

            assert_eq!(way_b.nodes.len(), 2);
            assert!(std::ptr::eq(&*way_b.get_node(0), &*node_a));
            assert!(std::ptr::eq(&*way_b.get_node(1), &*node_b));
        },
        Err(_) => {
            assert!(false);
        }
    }
}


#[test]
fn test_relation_parsing() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 0.0,
                "lon": 0.0
            },
            {
                "type": "node",
                "id": 2,
                "lat": 1.0,
                "lon": 2.0
            },
            {
                "type": "node",
                "id": 3,
                "lat": 1.0,
                "lon": 3.0
            },
            {
                "type": "node",
                "id": 4,
                "lat": 2.5,
                "lon": 3.8
            },
            {
                "type": "way",
                "id": 5,
                "nodes": [
                    1,
                    2,
                    3
                ]
            },
            {
                "type": "relation",
                "id": 6,
                "members": [
                    {
                        "type": "way",
                        "ref": 5,
                        "role": ""
                    },
                    {
                        "type": "node",
                        "ref": 4,
                        "role": ""
                    },
                    {
                        "type": "node",
                        "ref": 3,
                        "role": ""
                    }
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            assert_eq!(map_data.nodes.len(), 4);
            assert_eq!(map_data.ways.len(), 1);
            assert_eq!(map_data.relations.len(), 1);

            let node_a = map_data.get_node(1);
            let node_b = map_data.get_node(2);
            let node_c = map_data.get_node(3);
            let node_d = map_data.get_node(4);

            let way = map_data.get_way(5);

            let relation = map_data.get_relation(6);

            assert_eq!(relation.id, 6);

            assert_eq!(node_a.relations.len(), 0);
            assert_eq!(node_a.ways.len(), 1);

            assert_eq!(node_b.relations.len(), 0);
            assert_eq!(node_b.ways.len(), 1);

            assert_eq!(node_c.relations.len(), 1);
            assert_eq!(node_c.ways.len(), 1);
            assert!(std::ptr::eq(&*node_c.get_parent_relation(0), &*relation));

            assert_eq!(node_d.relations.len(), 1);
            assert_eq!(node_d.ways.len(), 0);
            assert!(std::ptr::eq(&*node_d.get_parent_relation(0), &*relation));

            let way_parent_relation = way.relations[0].borrow();
            assert!(std::ptr::eq(&*way_parent_relation, &*relation));

            let node_parent_relation = way.relations[0].borrow();
            assert!(std::ptr::eq(&*node_parent_relation, &*relation));

            assert_eq!(relation.members.nodes.len(), 2);
            assert_eq!(relation.members.ways.len(), 1);
            assert_eq!(relation.members.relations.len(), 0);

            assert!(std::ptr::eq(&*relation.get_node(0), &*node_d));
            assert!(std::ptr::eq(&*relation.get_node(1), &*node_c));
            assert!(std::ptr::eq(&*relation.get_way(0), &*way));

        },
        Err(_) => {
            assert!(false);
        }
    }
}


#[test]
fn test_relation_of_relations_parsing() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "relation",
                "id": 1,
                "members": [
                ]
            },
            {
                "type": "relation",
                "id": 2,
                "members": [
                    {
                        "type": "relation",
                        "ref": 1,
                        "role": ""
                    }
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            assert_eq!(map_data.nodes.len(), 0);
            assert_eq!(map_data.ways.len(), 0);
            assert_eq!(map_data.relations.len(), 2);

            let relation_a = map_data.get_relation(1);
            let relation_b = map_data.get_relation(2);

            assert_eq!(relation_a.members.nodes.len(), 0);
            assert_eq!(relation_a.members.ways.len(), 0);
            assert_eq!(relation_a.members.relations.len(), 0);
            assert_eq!(relation_a.relations.len(), 1);

            assert_eq!(relation_b.members.nodes.len(), 0);
            assert_eq!(relation_b.members.ways.len(), 0);
            assert_eq!(relation_b.members.relations.len(), 1);
            assert_eq!(relation_b.relations.len(), 0);

            let parent_relation = relation_a.get_parent_relation(0);
            assert!(std::ptr::eq(&*parent_relation, &*relation_b));

            let child_relation = relation_b.get_relation(0);
            assert!(std::ptr::eq(&*child_relation, &*relation_a));
        },
        Err(_) => {
            assert!(false);
        }
    }
}


#[test]
fn test_tag_parsing() {

}


#[test]
fn test_json_parser_fail() {

    let not_json_data = "This is not JSON!";

    match parser::from_string(not_json_data) {
        Ok(_) => {
            assert!(false);
        }
        Err(_) => {
            assert!(true);
        }
    }
}
