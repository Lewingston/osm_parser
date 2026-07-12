
use crate::map::{
    Relation,
    RelationMembers,
    RelationNode,
    RelationWay,
    RelationRelation,
    RelationMemberRole
};

use std::str::FromStr;

use serde_json::Value;

use crate::parser::tags;

type JsonObj = serde_json::Map<String, Value>;
type JsonArray = Vec<Value>;


enum RelationMember {
    Undefined,
    Node(RelationNode),
    Way(RelationWay),
    Relation(RelationRelation)
}


pub fn parse(relation: &JsonObj) -> Option<Relation> {

    let Some(id) = relation.get("id").and_then(Value::as_u64) else {
        println!("Relation has no id!");
        return None
    };

    let tags = relation.get("tags").and_then(tags::parse);

    let Some(member_array) = relation.get("members").and_then(Value::as_array) else {
        println!("Relation has no members");
        return None
    };

    let members = parse_members(member_array);

    Some(Relation {
        id,
        tags,
        members
    })
}


fn parse_members(members: &JsonArray) -> RelationMembers {

    let mut result = RelationMembers {
        nodes:     Vec::<RelationNode>::new(),
        ways:      Vec::<RelationWay>::new(),
        relations: Vec::<RelationRelation>::new()
    };

    for member in members {

        let Some(member_obj) = member.as_object() else {
            println!("Relation member is not an JSON object!");
            continue;
        };

        match parse_member(member_obj) {

            RelationMember::Node(node) => {
                result.nodes.push(node);
            },
            RelationMember::Way(way) => {
                result.ways.push(way);
            },
            RelationMember::Relation(relation) => {
                result.relations.push(relation);
            },
            RelationMember::Undefined => {
                println!("Unable to parse relation member!");
            }
        }
    }

    result
}


fn parse_member(data: &JsonObj) -> RelationMember {

    let Some(member_type) = data.get("type").and_then(Value::as_str) else {
        println!("Relation member has no type!");
        return RelationMember::Undefined;
    };

    let role_string = data.get("role").and_then(Value::as_str).unwrap_or("");

    let role = if let Ok(role) = RelationMemberRole::from_str(role_string) { role } else {
        println!("Unknown relation member role: {role_string}");
        RelationMemberRole::None
    };

    let Some(id) = data.get("ref").and_then(Value::as_u64) else {
        println!("Relation member has no id!");
        return RelationMember::Undefined;
    };

    match member_type {
        "node" => {
            RelationMember::Node(RelationNode{node: None, id, role})
        },
        "way" => {
            RelationMember::Way(RelationWay{way: None, id, role})
        },
        "relation" => {
            RelationMember::Relation(RelationRelation{relation: None, id, role})
        }
        _ => { RelationMember::Undefined }
    }
}
