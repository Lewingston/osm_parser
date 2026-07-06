
mod parser;

use std::collections::HashSet;

use osm_parser::map::Feature;
//use osm_parser::map::FeatureSubType;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = parser::from_file("bingen.json")?;

    let nodes_with_tags = data.nodes.iter().filter(|(_, node)| node.tags.is_some()).count();
    let nodes_without_tags = data.nodes.iter().filter(|(_, node)| node.tags.is_none()).count();

    println!("Node count: {}", data.nodes.len());
    println!("    Nodes with tags: {nodes_with_tags}");
    println!("    Nodes without tags: {nodes_without_tags}");

    let ways_with_tags = data.ways.iter().filter(|(_, way)| way.tags.is_some()).count();
    let ways_without_tags = data.ways.iter().filter(|(_, way)| way.tags.is_none()).count();

    println!("Way count: {}", data.ways.len());
    println!("    Ways with tags: {ways_with_tags}");
    println!("    Ways without tags: {ways_without_tags}");

    let rel_with_tags = data.relations.iter().filter(|(_, rel)| rel.tags.is_some()).count();
    let rel_without_tags = data.relations.iter().filter(|(_, rel)| rel.tags.is_none()).count();

    println!("Relation count: {}", data.relations.len());
    println!("    Relations with tags: {rel_with_tags}");
    println!("    Relations without tags: {rel_without_tags}");

    let mut features = HashSet::<Feature>::new();

    let mut num_nodes_without_feat     = 0;
    let mut num_ways_without_feat      = 0;
    let mut num_relations_without_feat = 0;

    for (_, node) in data.nodes {

        let Some(tags) = node.tags else { continue };

        if tags.features.is_empty() {
            num_nodes_without_feat += 1;
        }

        for feat in tags.features {
            features.insert(feat);
        }
    }

    for (_, way) in data.ways {

        let Some(tags) = way.tags else { continue };

        if tags.features.is_empty() {
            num_ways_without_feat += 1;
        }

        for feat in tags.features {
            features.insert(feat);
        }
    }

    for (_, relation) in data.relations {

        let Some(tags) = relation.tags else { continue };

        if tags.features.is_empty() {
            num_relations_without_feat += 1;
        }

        for feat in tags.features {
            features.insert(feat);
        }
    }

    println!("Number of different features: {}", features.len());
    println!("Number of nodes without feature: {num_nodes_without_feat}");
    println!("Number of ways without feature: {num_ways_without_feat}");
    println!("Number of relations without feature: {num_relations_without_feat}");

    /*
    for feat in features {

        println!("{}", feat.subtype_to_string());
    }
    */

    /*
    println!("Relations without map feature:");
    for (_, relation) in data.relations {

        let Some(tags) = relation.tags else { continue };

        if tags.features.is_empty() {
            println!("Relation: {}", relation.id);
        }
    }
    */

    Ok(())
}
