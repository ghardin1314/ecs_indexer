use crate::prelude::*;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigData {
    pub chain: Chain,
    // TODO: Make optional with defaults
    pub reorg_length: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ContractData {
    pub abi: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct EventTriggerData {
    pub signature: String,
    pub source_actions: Option<Vec<SourceActionData>>,
    pub notify_actions: Option<Vec<NotifyActionData>>,
}

#[derive(Debug, PartialEq, Deserialize)]

pub struct SourceActionData {
    pub template: String,
    pub param: String,
}

#[derive(Debug, PartialEq, Deserialize)]

pub struct NotifyActionData {
    pub endpoint: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SourceData {
    pub contract: ContractData,
    pub address: Option<Address>,
    pub start_block: Option<usize>,
    pub event_triggers: Option<Vec<EventTriggerData>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TemplateData {
    pub contract: ContractData,
    pub event_triggers: Option<Vec<EventTriggerData>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Manifest {
    pub config: ConfigData,
    pub sources: Vec<SourceData>,
    pub templates: Option<Vec<TemplateData>>,
}
