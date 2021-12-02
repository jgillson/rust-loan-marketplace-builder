use serde::{Deserialize, Serialize};
use crate::aggregate::Lender;

// LenderGroupCommand enum with three elements (AddLenderGroup, AddLender, RemoveLender)
#[derive(Serialize, Deserialize)]
pub enum LenderGroupCommand {
    AddLenderGroup(AddLenderGroup),
    AddLender(AddLender),
    RemoveLender(RemoveLender),
}

// AddLenderGroup struct to represent AddLenderGroup payload
#[derive(Serialize, Deserialize)]
pub struct AddLenderGroup {
    pub lender_group_id: String,
    pub lender_group_name: String
}

// AddLender struct to represent AddLender payload
#[derive(Serialize, Deserialize)]
pub struct AddLender {
    pub lender: Lender
}

// RemoveLender struct to represent RemoveLender payload
#[derive(Serialize, Deserialize)]
pub struct RemoveLender {
    pub lender: Lender
}
