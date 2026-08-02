
use crate::parser::pbf::protos;
use crate::parser::pbf::protos::{
    osmformat::PrimitiveBlock
};
use protobuf::Message;

use std::rc::Rc;
use std::cell::RefCell;

use crate::map::{
    Id,
    Node,
    Way,
    WayNode,
    Relation,
    RelationMap,
    RelationMembers,
    RelationNode,
    RelationWay,
    RelationRelation,
    RelationMemberRole
};

use crate::parser::pbf::error::OsmBlockError;
use crate::parser::pbf::string_table::StringTable;

use std::str::FromStr;

pub type Nodes     = Vec<Rc<RefCell<Node>>>;
pub type Ways      = Vec<Rc<RefCell<Way>>>;
pub type Relations = Vec<Rc<RefCell<Relation>>>;


pub struct BlockData {

    pub nodes:     Nodes,
    pub ways:      Ways,
    pub relations: Relations
}


impl BlockData {

    fn new() -> Self {

        Self {
            nodes:     Nodes::new(),
            ways:      Ways::new(),
            relations: Relations::new()
        }
    }
}


struct PrimitiveBlockContext<'block> {

    string_table:      StringTable<'block>,
    granularity:       i32,
    lat_offset:        i64,
    lon_offset:        i64,
    _date_granularity: i32,
}


pub fn parse(data: &[u8]) -> Result<Vec<BlockData>, Box<dyn std::error::Error>> {

    let block = match PrimitiveBlock::parse_from_bytes(data) {
        Ok(block) => { block },
        Err(err)  => { return Err(Box::new(err)); }
    };

    Ok(block.parse()?)
}


trait PrimitiveBlockEx {

    fn parse(&self) -> Result<Vec<BlockData>, OsmBlockError>;
}


impl PrimitiveBlockEx for PrimitiveBlock {

    fn parse(&self) -> Result<Vec<BlockData>, OsmBlockError> {

        let string_table = StringTable::new(&self.stringtable);

        let context = PrimitiveBlockContext {
            string_table,
            granularity:       self.granularity.unwrap_or(100),
            lat_offset:        self.lat_offset.unwrap_or(0),
            lon_offset:        self.lon_offset.unwrap_or(0),
            _date_granularity: self.date_granularity.unwrap_or(1000)
        };

        let mut result = Vec::<BlockData>::with_capacity(self.primitivegroup.len());

        for group in &self.primitivegroup {

            result.push(group.parse(&context)?);
        }

        Ok(result)
    }
}


trait PrimitiveGroupEx {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError>;
}


impl PrimitiveGroupEx for protos::osmformat::PrimitiveGroup {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<BlockData, OsmBlockError> {

        let mut result = BlockData::new();

        if let Some(dense_nodes) = &self.dense.0 {

            result.nodes = dense_nodes.parse(context)?;
        }

        for node in &self.nodes {

            result.nodes.push(node.parse(context)?);
        }

        for way in &self.ways {

            result.ways.push(way.parse(context)?);
        }

        for relation in &self.relations {

            result.relations.push(relation.parse(context)?);
        }

        if !self.changesets.is_empty() {
            return Err(OsmBlockError::UnsupportedAttribute("changesets", "PrimitiveGroup"));
        }

        Ok(result)
    }
}


trait OsmFormatNodeExt {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Rc<RefCell<Node>>, OsmBlockError>;
}


impl OsmFormatNodeExt for protos::osmformat::Node {

     fn parse(&self, _context: &PrimitiveBlockContext) -> Result<Rc<RefCell<Node>>, OsmBlockError> {

         Err(OsmBlockError::ParserNotImplemented("Node"))
     }
}


trait DenseNodesExt {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Nodes, OsmBlockError>;
}


impl DenseNodesExt for protos::osmformat::DenseNodes {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Nodes, OsmBlockError> {

        // TODO: Parse DenseInfo

        if self.id.len() != self.lat.len() ||
           self.id.len() != self.lon.len() {

            return Err(OsmBlockError::WrongNumberOfAttributes(
                format!("DenseNode - Id: {} - Lat: {} - Lon: {}",
                    self.id.len(),
                    self.lat.len(),
                    self.lon.len())
            ));
        }

        let mut nodes = Nodes::new();

        let mut keys_vals = self.keys_vals.split(|&i| i == 0);

        let mut id = Id(0);
        let mut latitude:  f64 = 0.0;
        let mut longitude: f64 = 0.0;

        for index in 0..self.id.len() {

            let Some(keys_vals) = keys_vals.next() else {
                return Err(OsmBlockError::DenseNodeKeysValuesError);
            };

            let tags = context.string_table.get_dense_node_tags(keys_vals)?;

            id = delta_id(id, self.id[index])?;

            let granularity = i64::from(context.granularity);
            let lat = 0.000_000_001 * (context.lat_offset + (granularity * self.lat[index])) as f64;
            let lon = 0.000_000_001 * (context.lon_offset + (granularity * self.lon[index])) as f64;

            latitude  += lat;
            longitude += lon;

            let node = Node {
                id,
                latitude,
                longitude,
                tags: Some(tags),
                ..Default::default()
            };

            nodes.push(Rc::new(RefCell::new(node)));
        }

        Ok(nodes)
    }
}


trait OsmFormatWayExt {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Rc<RefCell<Way>>, OsmBlockError>;
}


impl OsmFormatWayExt for protos::osmformat::Way {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Rc<RefCell<Way>>, OsmBlockError> {

        let id = osm_id_from_opt_i64(self.id)?;

        let tags = context.string_table.get_tags(&self.keys, &self.vals)?;

        // TODO: Parse Info

        let mut child_nodes = Vec::<WayNode>::new();

        /*
        // There might to many ways without nodes to print them
        if child_nodes.is_empty() {
            println!("Way without members: {:#?}", id);
        }
        */

        let mut ref_id = Id(0);

        for node_id in &self.refs {

            ref_id = delta_id(ref_id, *node_id)?;
            child_nodes.push(WayNode { id: ref_id, node: None });
        }

        if !self.lat.is_empty() || !self.lon.is_empty() {
            return Err(OsmBlockError::ParserNotImplemented("LocationOnWays"));
        }

        Ok(Rc::new(RefCell::new(Way {
            id,
            tags: Some(tags),
            child_nodes,
            parent_relations: RelationMap::new()
        })))
   }
}


trait OsmFormatRelationExt {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Rc<RefCell<Relation>>, OsmBlockError>;

    fn get_members(
        &self,
        context: &PrimitiveBlockContext,
        relation_id: Id)
    -> Result<RelationMembers, OsmBlockError>;
}


impl OsmFormatRelationExt for protos::osmformat::Relation {

    fn parse(&self, context: &PrimitiveBlockContext) -> Result<Rc<RefCell<Relation>>, OsmBlockError> {

        let id = osm_id_from_opt_i64(self.id)?;

        let tags = context.string_table.get_tags(&self.keys, &self.vals)?;

        // TODO: Parse info

        let members = self.get_members(context, id)?;

        Ok(Rc::new(RefCell::new(Relation {
            id,
            tags: Some(tags),
            members,
            parent_relations: RelationMap::new()
        })))
    }


    fn get_members(
        &self,
        context: &PrimitiveBlockContext,
        relation_id: Id
    ) -> Result<RelationMembers, OsmBlockError> {

        use protos::osmformat::relation::MemberType;

        if self.memids.len() != self.roles_sid.len() ||
           self.memids.len() != self.types.len() {

            return Err(OsmBlockError::WrongNumberOfAttributes(
                format!("Relation members - Ids: {} - Roles: {} - Types: {}",
                    self.memids.len(),
                    self.roles_sid.len(),
                    self.types.len())
            ));
        }

        if self.memids.is_empty() {
            return Err(OsmBlockError::RelationWithoutMembers(relation_id.0));
        }

        let mut members = RelationMembers::new();

        let mut id = Id(0);

        for ((ref_id, role_sid), type_) in
            self.memids.iter()
            .zip(self.roles_sid.iter())
            .zip(self.types.iter()) {

            id = delta_id(id, *ref_id)?;

            let Ok(role_sid) = (*role_sid).try_into() else {
                return Err(OsmBlockError::InvalidOsmId(format!("{}", *role_sid)));
            };

            let Some(role) = context.string_table.get(role_sid) else {
                return Err(OsmBlockError::StringTableAccess(role_sid));
            };

            let role = match RelationMemberRole::from_str(role) {
                Ok(role) => { role },
                Err(_) => {
                    println!("Unknown relation member role: {:#?}", role);
                    RelationMemberRole::None
                }
            };

            match type_.enum_value() {
                Ok(MemberType::NODE) => {

                    members.nodes.push(RelationNode {
                        node: None,
                        id,
                        role
                    });
                }
                Ok(MemberType::WAY) => {

                    members.ways.push(RelationWay {
                        way: None,
                        id,
                        role
                    });
                }
                Ok(MemberType::RELATION) => {

                    members.relations.push(RelationRelation {
                        relation: None,
                        id,
                        role
                    });
                }
                Err(value) => {
                    return Err(OsmBlockError::UnknownRelationTypeId(value));
                }
            }
        }

        Ok(members)
    }
}


fn delta_id(id: Id, delta: i64) -> Result<Id, OsmBlockError> {

    if delta < 0 && delta.abs() as u64 > id.0 {
        Err(OsmBlockError::OsmIdUnderflow(id.0, delta))
    } else {
        Ok(id + delta)
    }
}


fn osm_id_from_opt_i64(opt_id: Option<i64>) -> Result<Id, OsmBlockError> {

    let Some(id) = opt_id else {
        return Err(OsmBlockError::MissingAttribute("id"));
    };

    let Ok(id) = id.try_into() else {
        return Err(OsmBlockError::InvalidOsmId(format!("{id}")));
    };

    Ok(Id(id))
}
