
use osm_parser::parser;
use osm_parser::map::Id;


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

            let id = Id(1);
            let node = map_data.get_node(id).unwrap();

            assert_eq!(node.id, id);
            assert_eq!(node.latitude, 1.0);
            assert_eq!(node.longitude, 3.0);
            assert_eq!(node.parent_ways.len(), 0);
            assert_eq!(node.parent_relations.len(), 0);

        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
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

            let node_a = map_data.get_node(Id(1)).unwrap();
            let node_b = map_data.get_node(Id(2)).unwrap();
            let node_c = map_data.get_node(Id(3)).unwrap();

            let way_a = map_data.get_way(Id(4)).unwrap();
            let way_b = map_data.get_way(Id(5)).unwrap();

            assert_eq!(node_a.parent_ways.len(), 2);
            assert!(std::ptr::eq(&*node_a.get_parent_way(Id(4)).unwrap(), &*way_a));
            assert!(std::ptr::eq(&*node_a.get_parent_way(Id(5)).unwrap(), &*way_b));

            assert_eq!(node_b.parent_ways.len(), 2);
            assert!(std::ptr::eq(&*node_b.get_parent_way(Id(4)).unwrap(), &*way_a));
            assert!(std::ptr::eq(&*node_b.get_parent_way(Id(5)).unwrap(), &*way_b));

            assert_eq!(node_c.parent_ways.len(), 1);
            assert!(std::ptr::eq(&*node_c.get_parent_way(Id(4)).unwrap(), &*way_a));

            assert_eq!(way_a.id, Id(4));
            assert_eq!(way_b.id, Id(5));

            assert_eq!(way_a.child_nodes.len(), 3);
            assert!(std::ptr::eq(&*way_a.get_child_node(0).unwrap(), &*node_c));
            assert!(std::ptr::eq(&*way_a.get_child_node(1).unwrap(), &*node_b));
            assert!(std::ptr::eq(&*way_a.get_child_node(2).unwrap(), &*node_a));

            assert_eq!(way_b.child_nodes.len(), 2);
            assert!(std::ptr::eq(&*way_b.get_child_node(0).unwrap(), &*node_a));
            assert!(std::ptr::eq(&*way_b.get_child_node(1).unwrap(), &*node_b));

            assert!(way_a.is_complete());
            assert!(way_b.is_complete());
        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
        }
    }
}


#[test]
fn test_incomplete_way_parsing() {

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
                "type": "way",
                "id": 2,
                "nodes": [
                    10,
                    1,
                    11,
                    12
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            let node = map_data.get_node(Id(1)).unwrap();
            let way  = map_data.get_way(Id(2)).unwrap();

            assert_eq!(node.parent_ways.len(), 1);
            assert_eq!(way.child_nodes.len(), 4);

            assert_eq!(way.child_nodes[0].id, Id(10));
            assert!(way.get_child_node(0).is_none());

            assert_eq!(way.child_nodes[1].id, Id(1));
            assert!(std::ptr::eq(&*way.get_child_node(1).unwrap(), &*node));

            assert!(!way.is_complete());
        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
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

            let node_a = map_data.get_node(Id(1)).unwrap();
            let node_b = map_data.get_node(Id(2)).unwrap();
            let node_c = map_data.get_node(Id(3)).unwrap();
            let node_d = map_data.get_node(Id(4)).unwrap();

            let way = map_data.get_way(Id(5)).unwrap();

            let relation = map_data.get_relation(Id(6)).unwrap();

            assert_eq!(relation.id, Id(6));

            assert_eq!(node_a.parent_relations.len(), 0);
            assert_eq!(node_a.parent_ways.len(), 1);

            assert_eq!(node_b.parent_relations.len(), 0);
            assert_eq!(node_b.parent_ways.len(), 1);

            assert_eq!(node_c.parent_relations.len(), 1);
            assert_eq!(node_c.parent_ways.len(), 1);
            assert!(std::ptr::eq(&*node_c.get_parent_relation(Id(6)).unwrap(), &*relation));

            assert_eq!(node_d.parent_relations.len(), 1);
            assert_eq!(node_d.parent_ways.len(), 0);
            assert!(std::ptr::eq(&*node_d.get_parent_relation(Id(6)).unwrap(), &*relation));

            let way_parent_relation = way.get_parent_relation(Id(6)).unwrap();
            assert!(std::ptr::eq(&*way_parent_relation, &*relation));

            let node_parent_relation = way.get_parent_relation(Id(6)).unwrap();
            assert!(std::ptr::eq(&*node_parent_relation, &*relation));

            assert_eq!(relation.members.nodes.len(), 2);
            assert_eq!(relation.members.ways.len(), 1);
            assert_eq!(relation.members.relations.len(), 0);

            assert!(std::ptr::eq(&*relation.get_child_node(0).unwrap(), &*node_d));
            assert!(std::ptr::eq(&*relation.get_child_node(1).unwrap(), &*node_c));
            assert!(std::ptr::eq(&*relation.get_child_way(0).unwrap(), &*way));

            assert!(relation.is_complete());

        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
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

            let relation_a = map_data.get_relation(Id(1)).unwrap();
            let relation_b = map_data.get_relation(Id(2)).unwrap();

            assert_eq!(relation_a.members.nodes.len(), 0);
            assert_eq!(relation_a.members.ways.len(), 0);
            assert_eq!(relation_a.members.relations.len(), 0);
            assert_eq!(relation_a.parent_relations.len(), 1);

            assert_eq!(relation_b.members.nodes.len(), 0);
            assert_eq!(relation_b.members.ways.len(), 0);
            assert_eq!(relation_b.members.relations.len(), 1);
            assert_eq!(relation_b.parent_relations.len(), 0);

            let parent_relation = relation_a.get_parent_relation(Id(2)).unwrap();
            assert!(std::ptr::eq(&*parent_relation, &*relation_b));

            let child_relation = relation_b.get_child_relation(0).unwrap();
            assert!(std::ptr::eq(&*child_relation, &*relation_a));
        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
        }
    }
}


#[test]
fn incomplete_relation_parsing() {

    let json_data_missing_node = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 0.0,
                "lon": 0.0
            },
            {
                "type": "way",
                "id": 2,
                "nodes": [
                    1
                ]
            },
            {
                "type": "relation",
                "id": 3,
                "members": [
                    {
                        "type": "node",
                        "ref": 1,
                        "role": ""
                    }
                ]
            },
            {
                "type": "relation",
                "id": 4,
                "members": [
                    {
                        "type": "node",
                        "ref": 100,
                        "role": ""
                    },
                    {
                        "type": "way",
                        "ref": 2,
                        "role": ""
                    },
                    {
                        "type": "relation",
                        "ref": 3,
                        "role": ""
                    }
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data_missing_node) {
        Ok(map_data) => {

            let relation       = map_data.get_relation(Id(4)).unwrap();
            let other_relation = map_data.get_relation(Id(3)).unwrap();
            let way            = map_data.get_way(Id(2)).unwrap();

            assert_eq!(other_relation.parent_relations.len(), 1);
            assert_eq!(way.parent_relations.len(), 1);

            assert_eq!(relation.members.nodes.len(), 1);
            assert_eq!(relation.members.nodes[0].id, Id(100));
            assert!(relation.members.nodes[0].node.is_none());

            assert_eq!(relation.members.ways.len(), 1);
            assert!(std::ptr::eq(&*relation.get_child_way(0).unwrap(), &*way));

            assert_eq!(relation.members.relations.len(), 1);
            assert!(std::ptr::eq(&*relation.get_child_relation(0).unwrap(), &*other_relation));

            assert!(!relation.is_complete());

        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
        }
    }

    let json_data_missing_way = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 0.0,
                "lon": 0.0
            },
            {
                "type": "relation",
                "id": 2,
                "members": [
                    {
                        "type": "node",
                        "ref": 1,
                        "role": ""
                    }
                ]
            },
            {
                "type": "relation",
                "id": 3,
                "members": [
                    {
                        "type": "node",
                        "ref": 1,
                        "role": ""
                    },
                    {
                        "type": "way",
                        "ref": 333,
                        "role": ""
                    },
                    {
                        "type": "relation",
                        "ref": 2,
                        "role": ""
                    }
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data_missing_way) {
        Ok(map_data) => {

            let relation       = map_data.get_relation(Id(3)).unwrap();
            let other_relation = map_data.get_relation(Id(2)).unwrap();
            let node           = map_data.get_node(Id(1)).unwrap();

            assert_eq!(node.parent_relations.len(), 2);
            assert_eq!(other_relation.parent_relations.len(), 1);

            assert_eq!(relation.members.ways.len(), 1);
            assert_eq!(relation.members.ways[0].id, Id(333));
            assert!(relation.members.ways[0].way.is_none());

            assert_eq!(relation.members.nodes.len(), 1);
            assert!(std::ptr::eq(&*relation.get_child_node(0).unwrap(), &*node));

            assert_eq!(relation.members.relations.len(), 1);
            assert!(std::ptr::eq(&*relation.get_child_relation(0).unwrap(), &*other_relation));

            assert!(!relation.is_complete());
        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
        }
    }

    let json_data_missing_relation = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 0.0,
                "lon": 0.0
            },
            {
                "type": "way",
                "id": 2,
                "nodes": [
                    1
                ]
            },
            {
                "type": "relation",
                "id": 3,
                "members": [
                    {
                        "type": "node",
                        "ref": 1,
                        "role": ""
                    },
                    {
                        "type": "way",
                        "ref": 2,
                        "role": ""
                    },
                    {
                        "type": "relation",
                        "ref": 777,
                        "role": ""
                    }
                ]
            }
        ]
    }
    "#;

    match parser::from_string(json_data_missing_relation) {
        Ok(map_data) => {

            let relation = map_data.get_relation(Id(3)).unwrap();
            let node     = map_data.get_node(Id(1)).unwrap();
            let way      = map_data.get_way(Id(2)).unwrap();

            assert_eq!(node.parent_relations.len(), 1);
            assert_eq!(way.parent_relations.len(), 1);

            assert_eq!(relation.members.relations.len(), 1);
            assert_eq!(relation.members.relations[0].id, Id(777));
            assert!(relation.members.relations[0].relation.is_none());

            assert_eq!(relation.members.nodes.len(), 1);
            assert!(std::ptr::eq(&*relation.get_child_node(0).unwrap(), &*node));

            assert_eq!(relation.members.ways.len(), 1);
            assert!(std::ptr::eq(&*relation.get_child_way(0).unwrap(), &*way));

            assert!(!relation.is_complete());
        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
        }
    }
}


#[test]
fn test_circular_relation_parsing() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "relation",
                "id": 1,
                "members": [
                    {
                        "type": "relation",
                        "ref": 2,
                        "role": ""
                    }
                ]
            },
            {
                "type": "relation",
                "id": 2,
                "members": [
                    {
                        "type": "relation",
                        "ref": 3,
                        "role": ""
                    }
                ]
            },
            {
                "type": "relation",
                "id": 3,
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

            let rel1 = map_data.get_relation(Id(1)).unwrap();
            let rel2 = map_data.get_relation(Id(2)).unwrap();
            let rel3 = map_data.get_relation(Id(3)).unwrap();

            assert_eq!(rel1.members.relations.len(), 1);
            assert_eq!(rel1.parent_relations.len(), 1);
            assert_eq!(rel2.members.relations.len(), 1);
            assert_eq!(rel2.parent_relations.len(), 1);
            assert_eq!(rel3.members.relations.len(), 1);
            assert_eq!(rel3.parent_relations.len() , 1);

            assert!(std::ptr::eq(&*rel1.get_child_relation(0).unwrap(), &*rel2));
            assert!(std::ptr::eq(&*rel1.get_parent_relation(Id(3)).unwrap(), &*rel3));

            assert!(std::ptr::eq(&*rel2.get_child_relation(0).unwrap(), &*rel3));
            assert!(std::ptr::eq(&*rel2.get_parent_relation(Id(1)).unwrap(), &*rel1));

            assert!(std::ptr::eq(&*rel3.get_child_relation(0).unwrap(), &*rel1));
            assert!(std::ptr::eq(&*rel3.get_parent_relation(Id(2)).unwrap(), &*rel2));

            assert!(rel1.is_complete());
            assert!(rel2.is_complete());
            assert!(rel3.is_complete());

        },
        Err(err) => {
            assert!(false, "{}", err.to_string());
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
