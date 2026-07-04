
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = parser::from_file("bingen.json")?;

    println!("Node count: {}", data.nodes.len());
    println!("Way count: {}", data.ways.len());
    println!("Relation count: {}", data.relations.len());

    Ok(())
}
