
use osm_parser::parser;

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

            let Some(way_a) = map_data.ways.get(&4) else { assert!(false); return; };
            let way_a = way_a.borrow();
            let Some(way_b) = map_data.ways.get(&5) else { assert!(false); return; };
            let way_b = way_b.borrow();

            assert_eq!(way_a.nodes.len(), 3);
            assert_eq!(way_b.nodes.len(), 2);

            let Some(node) = map_data.nodes.get(&1) else { assert!(false); return; };
            let node = node.borrow();

            assert_eq!(node.ways.len(), 2);
        },
        Err(_) => {
            assert!(false);
        }
    }
}


#[test]
fn test_relation_parsing() {

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
