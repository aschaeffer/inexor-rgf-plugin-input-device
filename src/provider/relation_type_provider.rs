use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;

use crate::model::relation_type::RelationType;
use crate::plugins::RelationTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/relations"]
struct InputDeviceRelationTypeAsset;

#[async_trait]
pub trait InputDeviceRelationTypeProvider: RelationTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct InputDeviceRelationTypeProviderImpl {}

interfaces!(InputDeviceRelationTypeProviderImpl: dyn RelationTypeProvider);

#[component]
impl InputDeviceRelationTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl InputDeviceRelationTypeProvider for InputDeviceRelationTypeProviderImpl {}

impl RelationTypeProvider for InputDeviceRelationTypeProviderImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        let mut relation_types = Vec::new();
        for file in InputDeviceRelationTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading relation_type from resource {}", filename);
            let asset = InputDeviceRelationTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let relation_type: RelationType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(relation_type) => relation_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            relation_types.push(relation_type);
        }
        relation_types
    }
}
