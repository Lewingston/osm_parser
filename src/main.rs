
pub mod parser;
pub mod map;

//use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //parse_pbf()?;
    parse_json()?;

    Ok(())
}


fn parse_pbf() -> Result<(), Box<dyn std::error::Error>> {

    let data = parser::pbf::from_file("bremen-260728.osm.pbf")?;

    println!("Blocks: {}",    data.blocks.len());
    println!("Nodes: {}",     data.map.nodes.len());
    println!("Ways: {}",      data.map.ways.len());
    println!("Relations: {}", data.map.relations.len());

    Ok(())
}


fn parse_json() -> Result<(), Box<dyn std::error::Error>> {

    let map = parser::json::from_file("bingen.json")?;

    println!("Nodes: {}",     map.nodes.len());
    println!("Ways: {}",      map.ways.len());
    println!("Relations: {}", map.relations.len());

    Ok(())
}
