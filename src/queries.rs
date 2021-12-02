use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};

use crate::aggregate::{LenderGroup, Lender};
use crate::events::LenderGroupEvent;

pub struct SimpleLoggingQueryProcessor {}

// Each CQRS platform should have one or more QueryProcessors where it will distribute committed events
// It is the responsibility of the QueryProcessor to update any interested queries
impl QueryProcessor<LenderGroup> for SimpleLoggingQueryProcessor {
    fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<LenderGroup>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

// LenderGroupQuery struct to represent query payload
#[derive(Debug, Serialize, Deserialize)]
pub struct LenderGroupQuery {
    lender_group_id: Option<String>,
    lender_group_name: Option<String>,
    lenders: Vec<Lender>,
}

// A Query is a read element in a CQRS system
// As events are emitted multiple downstream queries are updated to reflect the current state of the system
// A query may also be referred to as a 'view', the concepts are identical but 'query' is used here to conform with CQRS nomenclature
// Queries are generally serialized for persistence, usually in a standard database, but a query could utilize messaging platform or other asynchronous, eventually-consistent systems
impl Query<LenderGroup> for LenderGroupQuery {
    fn update(&mut self, event: &EventEnvelope<LenderGroup>) {
        match &event.payload {
            LenderGroupEvent::LenderGroupAdded(payload) => {
                self.lender_group_id = Option::from(payload.lender_group_id.clone());
                self.lender_group_name = Option::from(payload.lender_group_name.clone());
            }
            LenderGroupEvent::LenderAdded(payload) => {
                self.lenders.push(payload.lender.clone());
            }
            LenderGroupEvent::LenderRemoved(payload) => {
                let index = self.lenders.iter().position(|r| r == &payload.lender.clone()).unwrap();
                self.lenders.remove(index);
            }
        }
    }
}

// Returns the "default value" for a type
// Default values are often some kind of initial value, identity value, or anything else that may make sense as a default
impl Default for LenderGroupQuery {
    fn default() -> Self {
        LenderGroupQuery {
            lender_group_id: None,
            lender_group_name: None,
            lenders: Default::default(),
        }
    }
}
