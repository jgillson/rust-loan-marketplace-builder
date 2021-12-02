use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::aggregate::Lender;

// LenderGroupEvent enum with three elements (LenderGroupAdded, LenderAdded, LenderRemoved)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LenderGroupEvent {
    LenderGroupAdded(LenderGroupAdded),
    LenderAdded(LenderAdded),
    LenderRemoved(LenderRemoved),
}

// LenderGroupAdded struct to represent LenderGroupAdded payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LenderGroupAdded {
    pub lender_group_id: String,
    pub lender_group_name: String
}

// LenderAdded struct to represent LenderAdded payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LenderAdded {
    pub lender: Lender
}

// LenderRemoved struct to represent LenderRemoved payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LenderRemoved {
    pub lender: Lender
}

// A DomainEvent represents any business change in the state of an Aggregate
// DomainEvents are immutable and with event sourcing they are the source of truth
// To simplify serialization, an event should be an enum, and each element should have a payload
// By convention, the payload has the same name as the element
impl DomainEvent for LenderGroupEvent {}
