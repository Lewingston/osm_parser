
use osm_parser::parser;

#[test]
fn test_json_parser() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 1.0,
                "lon": 1.0
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            assert_eq!(map_data.nodes.len(), 1);
        },
        Err(_) => {
            assert!(false);
        }
    }
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
