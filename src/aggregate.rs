use cqrs_es::{Aggregate, AggregateError};
use serde::{Deserialize, Serialize};

use crate::commands::LenderGroupCommand;
use crate::events::{LenderGroupEvent, LenderGroupAdded, LenderAdded, LenderRemoved};

// LenderGroup struct stores related data attributes
// The compiler is capable of providing basic implementations for some traits via the #[derive] attribute
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LenderGroup {
    lender_group_id: String,
    lender_group_name: String,
    lenders: Vec<Lender>,
}

// Lender struct stores related data attributes
// The compiler is capable of providing basic implementations for some traits via the #[derive] attribute
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lender {
    pub id: String,
    pub name: String,
}

// Custom implementation for PartialEq trait for partial equality comparisons
impl PartialEq for Lender {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Implement CQRS Aggregate trait for LenderGroup
// Provides a name for the aggregate type to distinguish it from any other aggregate
impl Aggregate for LenderGroup {
    // An inbound command used to make changes in the state of the Aggregate
    type Command = LenderGroupCommand;
    // An event representing some change in state of the Aggregate
    type Event = LenderGroupEvent;

    // aggregate_type is a unique identifier for this aggregate
    fn aggregate_type() -> &'static str {
        "lender_group"
    }

    // handle inbound command and return a vector of events or an error
    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, AggregateError> {
        match command {
            LenderGroupCommand::AddLenderGroup(payload) => {
                let event_payload = LenderGroupAdded {
                    lender_group_id: payload.lender_group_id,
                    lender_group_name: payload.lender_group_name
                };
                Ok(vec![LenderGroupEvent::LenderGroupAdded(event_payload)])
            }
            LenderGroupCommand::AddLender(payload) => {
                let event_payload = LenderAdded {
                    lender: payload.lender,
                };
                Ok(vec![LenderGroupEvent::LenderAdded(event_payload)])
            }
            LenderGroupCommand::RemoveLender(payload) => {
                let event_payload = LenderRemoved {
                    lender: payload.lender
                };
                Ok(vec![LenderGroupEvent::LenderRemoved(event_payload)])
            }
        }
    }

    // update the aggregate's state with an event
    fn apply(&mut self, event: Self::Event) {
        match event {
            LenderGroupEvent::LenderGroupAdded(e) => {
                self.lender_group_id = e.lender_group_id.clone();
                self.lender_group_name = e.lender_group_name.clone();
            }
            LenderGroupEvent::LenderAdded(e) => {
                self.lenders.push(e.lender.clone())
            }
            LenderGroupEvent::LenderRemoved(e) => {
                let index = self.lenders.iter().position(|r| r == &e.lender.clone()).unwrap();
                self.lenders.remove(index);
            }
        }
    }
}

// Returns the "default value" for a type
// Default values are often some kind of initial value, identity value, or anything else that may make sense as a default
impl Default for LenderGroup {
    fn default() -> Self {
        LenderGroup {
            lender_group_id: "".to_string(),
            lender_group_name: "".to_string(),
            lenders: Default::default(),
        }
    }
}
