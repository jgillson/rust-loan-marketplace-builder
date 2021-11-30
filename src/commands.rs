use serde::{Deserialize, Serialize};
use crate::aggregate::Lender;

#[derive(Serialize, Deserialize)]
pub enum LenderGroupCommand {
    AddLenderGroup(AddLenderGroup),
    AddLender(AddLender),
    RemoveLender(RemoveLender),
}

#[derive(Serialize, Deserialize)]
pub struct AddLenderGroup {
    pub lender_group_id: String,
    pub lender_group_name: String
}

#[derive(Serialize, Deserialize)]
pub struct AddLender {
    pub lender: Lender
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLender {
    pub lender: Lender
}
