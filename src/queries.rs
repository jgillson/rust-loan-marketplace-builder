use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};

use crate::aggregate::{LenderGroup, Lender};
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
    lender_group_id: Option<String>,
    lender_group_name: Option<String>,
    lenders: Vec<Lender>,
}

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

impl Default for LenderGroupQuery {
    fn default() -> Self {
        LenderGroupQuery {
            lender_group_id: None,
            lender_group_name: None,
            lenders: Default::default(),
        }
    }
}
