use serde::{Deserialize, Serialize};
use crate::aggregate::Lender;

#[derive(Serialize, Deserialize)]
pub enum LenderGroupCommand {
    AddLender(Lender),
    RemoveLender(Lender),
}
