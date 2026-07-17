
use osm_parser::parser;
use std::rc::Rc;

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

            let Some(node) = map_data.nodes.get(&1) else { assert!(false); return; };
            let node = node.borrow();

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

            let node_a = map_data.nodes.get(&1).expect("node with id 1 is missing").borrow();
            let node_b = map_data.nodes.get(&2).expect("node with id 2 is missing").borrow();
            let node_c = map_data.nodes.get(&3).expect("node with id 3 is missing").borrow();

            assert_eq!(node_a.ways.len(), 2);
            assert_eq!(node_b.ways.len(), 2);
            assert_eq!(node_c.ways.len(), 1);


            let way_a = map_data.ways.get(&4).expect("way with id 4 is missing").borrow();
            let way_b = map_data.ways.get(&5).expect("way with id 5 is missing").borrow();

            assert_eq!(way_a.id, 4);
            assert_eq!(way_b.id, 5);

            assert_eq!(way_a.nodes.len(), 3);
            let way_node_a = way_a.nodes[0].node.as_ref().expect("way node at position 0 is missing").borrow();
            let way_node_b = way_a.nodes[1].node.as_ref().expect("way node at position 1 is missing").borrow();
            let way_node_c = way_a.nodes[2].node.as_ref().expect("way node at position 2 is missing").borrow();
            assert!(std::ptr::eq(&*way_node_a, &*node_c));
            assert!(std::ptr::eq(&*way_node_b, &*node_b));
            assert!(std::ptr::eq(&*way_node_c, &*node_a));

            assert_eq!(way_b.nodes.len(), 2);
            let way_node_a = way_b.nodes[0].node.as_ref().expect("way node at position 0 is missing").borrow();
            let way_node_b = way_b.nodes[1].node.as_ref().expect("way node at position 1 is missing").borrow();
            assert!(std::ptr::eq(&*way_node_a, &*node_a));
            assert!(std::ptr::eq(&*way_node_b, &*node_b));
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

            let node_a = map_data.nodes.get(&1).expect("node with id 1 is missing").borrow();
            let node_b = map_data.nodes.get(&2).expect("node with id 2 is missing").borrow();
            let node_c = map_data.nodes.get(&3).expect("node with id 3 is missing").borrow();
            let node_d = map_data.nodes.get(&4).expect("node with id 4 is missing").borrow();

            let way = map_data.ways.get(&5).expect("way with id 5 is missing").borrow();

            let relation = map_data.relations.get(&6).expect("relation with id 6 is missing").borrow();

            assert_eq!(node_a.relations.len(), 0);
            assert_eq!(node_a.ways.len(), 1);

            assert_eq!(node_b.relations.len(), 0);
            assert_eq!(node_b.ways.len(), 1);

            assert_eq!(node_c.relations.len(), 1);
            assert_eq!(node_c.ways.len(), 1);

            assert_eq!(node_d.relations.len(), 1);
            assert_eq!(node_d.ways.len(), 0);

            let way_parent_relation = way.relations[0].borrow();
            assert!(std::ptr::eq(&*way_parent_relation, &*relation));

            let node_parent_relation = way.relations[0].borrow();
            assert!(std::ptr::eq(&*node_parent_relation, &*relation));

            assert_eq!(relation.members.nodes.len(), 2);
            assert_eq!(relation.members.ways.len(), 1);
            assert_eq!(relation.members.relations.len(), 0);

            assert!(std::ptr::eq(&*relation.members.nodes[0].node.clone().unwrap().borrow(), &*node_d));
            assert!(std::ptr::eq(&*relation.members.nodes[1].node.clone().unwrap().borrow(), &*node_c));
            assert!(std::ptr::eq(&*relation.members.ways[0].way.clone().unwrap().borrow(), &*way));

        },
        Err(_) => {
            assert!(false);
        }
    }
}


#[test]
fn test_relation_of_relations_parsing() {

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
