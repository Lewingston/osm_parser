
use osm_parser::parser;
use osm_parser::map::Dimensions;

use approx::assert_relative_eq;

#[test]
fn test_dimensions() {

    let test = Dimensions {
        min_lat: 20.0,
        max_lat: 80.0,
        min_lon: 120.0,
        max_lon: 300.0
    };

    let (lon, lat) = test.get_center();
    assert_relative_eq!(lon, 210.0);
    assert_relative_eq!(lat, 50.0);

    assert_relative_eq!(test.get_width(), 180.0);

    assert_relative_eq!(test.get_height(), 60.0);
}


#[test]
fn test_map_dimensions() {

    let json_data = r#"
    {
        "elements": [
            {
                "type": "node",
                "id": 1,
                "lat": 5.0,
                "lon": 8.0
            },
            {
                "type": "node",
                "id": 2,
                "lat": -2.0,
                "lon": 1.0
            },
            {
                "type": "node",
                "id": 3,
                "lat": 7.0,
                "lon": -4.0
            }
        ]
    }
    "#;

    match parser::from_string(json_data) {
        Ok(map_data) => {

            let dimensions = map_data.get_dimensions();

            assert_relative_eq!(dimensions.min_lat, -2.0);
            assert_relative_eq!(dimensions.max_lat,  7.0);
            assert_relative_eq!(dimensions.min_lon, -4.0);
            assert_relative_eq!(dimensions.max_lon,  8.0);
        },
        Err(err) => {
            assert!(false, "{err}");
        }
    }
}
