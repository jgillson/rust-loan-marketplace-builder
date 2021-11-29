use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::aggregate::Lender;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LenderGroupEvent {
    LenderAdded(Lender),
    LenderRemoved(Lender),
}

impl DomainEvent for LenderGroupEvent {}
