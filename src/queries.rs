use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};

use crate::aggregate::LenderGroup;
use crate::aggregate::Lender;
use crate::events::LenderGroupEvent;

pub struct SimpleLoggingQueryProcessor {}

impl QueryProcessor<LenderGroup> for SimpleLoggingQueryProcessor {
    fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<LenderGroup>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LenderGroupQuery {
    id: Option<String>,
    name: Option<String>,
    lenders: Vec<Lender>,
}

impl Query<LenderGroup> for LenderGroupQuery {
    fn update(&mut self, event: &EventEnvelope<LenderGroup>) {
        match &event.payload {
            LenderGroupEvent::LenderAdded(payload) => {
                let lender_added = Lender {
                    id: payload.id.clone(),
                    name: payload.name.clone(),
                    lender_group_id: payload.lender_group_id.clone()
                };
                self.lenders.push(lender_added);
            }
            LenderGroupEvent::LenderRemoved(payload) => {
                let lender_removed = Lender {
                    id: payload.id.clone(),
                    name: payload.name.clone(),
                    lender_group_id: payload.lender_group_id.clone()
                };
                let index = self.lenders.iter().position(|r| r == &lender_removed).unwrap();
                self.lenders.remove(index);
            }
        }
    }
}

impl Default for LenderGroupQuery {
    fn default() -> Self {
        LenderGroupQuery {
            id: None,
            name: None,
            lenders: Default::default(),
        }
    }
}
