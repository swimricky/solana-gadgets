use std::{any::Any, collections::HashMap};

/// solana-gadgets sad deserialization tree
use crate::{
    datamap::{is_sadvalue_type, SadValue},
    errors::SadTreeError,
};
use lazy_static::*;
use yaml_rust::yaml::Yaml;

trait Node: std::fmt::Debug {
    /// Clone of the inbound yaml sad 'type'
    fn decl_type(&self) -> &String;
    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>);
    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>);
}

trait NodeWithChildren: Node {
    fn children(&self) -> &Vec<Box<dyn Node>>;
}

const SAD_YAML_TYPE: &str = "type";
const SAD_YAML_NAME: &str = "name";
const SAD_YAML_DESCRIPTOR: &str = "descriptor";
const SAD_YAML_SIZE_TYPE: &str = "size_type";
const SAD_YAML_CONTAINS: &str = "contains";
const SAD_YAML_FIELDS: &str = "fields";

#[derive(Debug)]
pub struct SadLeaf {
    sad_value_type: String,
}

impl SadLeaf {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if is_sadvalue_type(in_str) {
            Ok(Box::new(SadLeaf {
                sad_value_type: String::from(in_str),
            }))
        } else {
            Err(SadTreeError::UnknownType(String::from(in_str)))
        }
    }
}

impl Node for SadLeaf {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}
#[derive(Debug)]
pub struct SadNamedField {
    sad_field_name: String,
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadNamedField {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let desc = &in_yaml[SAD_YAML_DESCRIPTOR];
        let in_name = desc[SAD_YAML_NAME].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        array.push(parse(desc).unwrap());
        println!("NF Child {:?}", array);
        Ok(Box::new(SadNamedField {
            sad_field_name: String::from(in_name),
            sad_value_type: String::from(SAD_YAML_DESCRIPTOR),
            children: array,
        }))
    }

    fn name(&self) -> &String {
        &self.sad_field_name
    }
}

impl Node for SadNamedField {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}
impl NodeWithChildren for SadNamedField {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

#[derive(Debug)]
pub struct SadLengthPrefix {
    sad_value_type: String,
    sad_length_type: String,
    children: Vec<Box<dyn Node>>,
}
impl SadLengthPrefix {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let hmap = in_yaml.as_hash().unwrap();
        let in_str = in_yaml[SAD_YAML_SIZE_TYPE].as_str().unwrap();
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }
        let in_type_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let contains = &in_yaml[SAD_YAML_CONTAINS];
        match contains {
            Yaml::Array(lst) => {
                // println!("slp = {:?}", lst);
                array.push(parse(&lst[0]).unwrap());
                // for hl in lst {
                //     array.push(parse(hl).unwrap())
                // }
                Ok(Box::new(SadLengthPrefix {
                    sad_value_type: String::from(in_type_str),
                    sad_length_type: String::from(in_str),
                    children: array,
                }))
            }
            Yaml::Hash(map) => {
                array.push(parse(contains).unwrap());
                Ok(Box::new(SadLengthPrefix {
                    sad_value_type: String::from(in_type_str),
                    sad_length_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedHashMapOrArray),
        }
    }
}
impl Node for SadLengthPrefix {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}
impl NodeWithChildren for SadLengthPrefix {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}
#[derive(Debug)]
pub struct SadHashMap {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadHashMap {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let hmap = in_yaml.as_hash().unwrap();
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

        let mut array = Vec::<Box<dyn Node>>::new();
        let fields = &in_yaml[SAD_YAML_FIELDS];
        match fields {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadHashMap {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedHashMapFields),
        }
    }
}
impl Node for SadHashMap {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}

impl NodeWithChildren for SadHashMap {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

#[derive(Debug)]
pub struct SadStructure {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadStructure {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

        let mut array = Vec::<Box<dyn Node>>::new();
        let fields = &in_yaml[SAD_YAML_FIELDS];

        match fields {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadStructure {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedCStructFields),
        }
    }
}
impl Node for SadStructure {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}

impl NodeWithChildren for SadStructure {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

#[derive(Debug)]
pub struct SadVector {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadVector {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let hmap = in_yaml.as_hash().unwrap();
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

        let mut array = Vec::<Box<dyn Node>>::new();
        let contains = &in_yaml[SAD_YAML_CONTAINS];
        match contains {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadVector {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedVecContains),
        }
    }
}
impl Node for SadVector {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}

impl NodeWithChildren for SadVector {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

#[derive(Debug)]
pub struct SadTuple {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadTuple {
    fn from_yaml(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
        let hmap = in_yaml.as_hash().unwrap();
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

        let mut array = Vec::<Box<dyn Node>>::new();
        let fields = &in_yaml[SAD_YAML_FIELDS];
        match fields {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadTuple {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedTupleFields),
        }
    }
}
impl Node for SadTuple {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }
}

impl NodeWithChildren for SadTuple {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

#[derive(Debug)]
pub struct SadTree {
    yaml_decl_type: String,
    name: String,
    children: Vec<Box<dyn Node>>,
}

impl SadTree {
    pub fn new(in_yaml: &Yaml) -> Result<Self, SadTreeError> {
        let mut array = Vec::<Box<dyn Node>>::new();
        match &*in_yaml {
            Yaml::Hash(ref hmap) => {
                let (key, value) = hmap.front().unwrap();
                match value {
                    Yaml::Array(hlobjects) => {
                        for hl in hlobjects {
                            let (_, h1_value) = hl.as_hash().unwrap().front().unwrap();
                            array.push(parse(h1_value).unwrap());
                        }
                        Ok(Self {
                            yaml_decl_type: String::from("tree"),
                            children: array,
                            name: key.as_str().unwrap().to_string(),
                        })
                    }
                    _ => Err(SadTreeError::ExpectedArray),
                }
            }
            _ => Err(SadTreeError::ExpectedHashMap),
        }
    }
}

impl NodeWithChildren for SadTree {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

impl Node for SadTree {
    fn decl_type(&self) -> &String {
        &self.yaml_decl_type
    }

    fn deser_line(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        todo!()
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        for c in &self.children {
            c.deser(data, collection)
        }
    }
}

#[derive(Debug)]
pub struct Deseriaizer<'a> {
    yaml_declaration: &'a Yaml,
    sad_tree: SadTree,
}

impl<'a> Deseriaizer<'a> {
    fn new(in_yaml: &'a Yaml) -> Self {
        Self {
            yaml_declaration: in_yaml,
            sad_tree: SadTree::new(in_yaml).unwrap(),
        }
    }
    fn deser(&self, data: &mut &[u8]) -> HashMap<String, Box<dyn Any>> {
        let hm = HashMap::<String, Box<dyn Any>>::new();
        hm
    }
    fn tree(&self) -> &SadTree {
        &self.sad_tree
    }
}
lazy_static! {
    static ref JUMP_TABLE: HashMap<String, fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError>> = {
        let mut jump_table =
            HashMap::<String, fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError>>::new();
        jump_table.insert("length_prefix".to_string(), SadLengthPrefix::from_yaml);
        jump_table.insert("HashMap".to_string(), SadHashMap::from_yaml);
        jump_table.insert("Vec".to_string(), SadVector::from_yaml);
        jump_table.insert("Tuple".to_string(), SadTuple::from_yaml);
        jump_table.insert("CStruct".to_string(), SadStructure::from_yaml);
        jump_table.insert("NamedField".to_string(), SadNamedField::from_yaml);
        jump_table.insert("other".to_string(), SadLeaf::from_yaml);
        jump_table
    };
}

fn parse(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
    let default = JUMP_TABLE.get("other").unwrap();
    // Expects a Hash construct and first entry
    let type_in = in_yaml.as_hash().unwrap().front().unwrap().1;
    if let Some(s) = JUMP_TABLE.get(type_in.as_str().unwrap()) {
        s(in_yaml)
    } else {
        default(in_yaml)
    }
}

#[cfg(test)]
mod tests {
    use gadgets_common::load_yaml_file;
    use strum::VariantNames;
    use yaml_rust::YamlLoader;

    use super::*;
    fn body_parse(in_yaml: &Yaml) -> Result<(), SadTreeError> {
        match &*in_yaml {
            Yaml::Real(_) => Ok(()),
            Yaml::Integer(_) => Ok(()),
            Yaml::String(_) => Ok(()),
            Yaml::Boolean(_) => Ok(()),
            Yaml::Array(_) => Ok(()),
            Yaml::Hash(_) => Ok(()),
            Yaml::Alias(_) => Ok(()),
            Yaml::Null => Ok(()),
            Yaml::BadValue => Ok(()),
            _ => Err(SadTreeError::UnknownType("".to_string())),
        }
    }
    #[test]
    fn test_leaf_node_pass() {
        for v in SadValue::VARIANTS.iter() {
            let vs = *v;
            let d = format!("{}: {}", "type", vs);
            let docs = YamlLoader::load_from_str(&d).unwrap();
            let doc = &docs[0]; // select the first document
            let sl = SadLeaf::from_yaml(doc);
            assert!(sl.is_ok());
            println!("{:?}", sl);
        }
    }

    #[test]
    fn test_scalars_pass() {
        let mut pos = 0;
        let pos_end = 14;
        for v in SadValue::VARIANTS.iter() {
            if pos == pos_end {
                break;
            }
            pos += 1;
            let vs = *v;
            let d = format!("{}: {}", "type", vs);
            let docs = YamlLoader::load_from_str(&d).unwrap();
            let result = parse(&docs[0]);
            assert!(result.is_ok());
        }
    }
    #[test]
    fn test_runner_pass() {
        // let result = load_yaml_file("../yaml_samps/test.yml").unwrap();
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
        for body in result {
            println!("{:?}", Deseriaizer::new(&body).tree());
        }
    }

    #[test]
    fn test_deserialization_pass() {
        let result = load_yaml_file("../yaml_samps/test.yml").unwrap();
        let desc = Deseriaizer::new(&result[0]);
    }
}