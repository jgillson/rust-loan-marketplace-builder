use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::aggregate::Lender;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LenderGroupEvent {
    LenderGroupAdded(LenderGroupAdded),
    LenderAdded(LenderAdded),
    LenderRemoved(LenderRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LenderGroupAdded {
    pub lender_group_id: String,
    pub lender_group_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LenderAdded {
    pub lender: Lender
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LenderRemoved {
    pub lender: Lender
}

impl DomainEvent for LenderGroupEvent {}
