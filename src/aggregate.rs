use cqrs_es::{Aggregate, AggregateError};
use serde::{Deserialize, Serialize};

use crate::commands::LenderGroupCommand;
use crate::events::LenderGroupEvent;

#[derive(Serialize, Deserialize)]
pub struct LenderGroup {
    id: String,
    name: String,
    lenders: Vec<Lender>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lender {
    pub id: String,
    pub name: String,
    pub lender_group_id: String,
}

impl PartialEq for Lender {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Aggregate for LenderGroup {
    type Command = LenderGroupCommand;
    type Event = LenderGroupEvent;

    fn aggregate_type() -> &'static str {
        "lender_group"
    }

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, AggregateError> {
        match command {
            LenderGroupCommand::AddLender(payload) => {
                let event_payload = Lender {
                    id: payload.id,
                    name: payload.name,
                    lender_group_id: payload.lender_group_id,
                };
                Ok(vec![LenderGroupEvent::LenderAdded(event_payload)])
            }
            LenderGroupCommand::RemoveLender(payload) => {
                let event_payload = Lender {
                    id: payload.id,
                    name: payload.name,
                    lender_group_id: payload.lender_group_id,
                };
                Ok(vec![LenderGroupEvent::LenderRemoved(event_payload)])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            LenderGroupEvent::LenderAdded(e) => {
                let lender_added = Lender {
                    id: e.id.clone(),
                    name: e.name.clone(),
                    lender_group_id: e.lender_group_id.clone()
                };
                self.lenders.push(lender_added)
            }
            LenderGroupEvent::LenderRemoved(e) => {
                let lender_removed = Lender {
                    id: e.id.clone(),
                    name: e.name.clone(),
                    lender_group_id: e.lender_group_id.clone()
                };
                let index = self.lenders.iter().position(|r| r == &lender_removed).unwrap();
                self.lenders.remove(index);
            }
        }
    }
}

impl Default for LenderGroup {
    fn default() -> Self {
        LenderGroup {
            id: "".to_string(),
            name: "".to_string(),
            lenders: Default::default(),
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use crate::aggregate::{LenderGroup, Lender};
    use crate::commands::LenderGroupCommand;
    use crate::events::LenderGroupEvent;

    type LenderGroupTestFramework = TestFramework<LenderGroup>;

    #[test]
    fn test_add_lender() {
        let expected = LenderGroupEvent::LenderAdded(Lender { id: 1.to_string(), name: "ABC Bank".to_string(), lender_group_id: 1.to_string() });

        LenderGroupTestFramework::default()
            .given_no_previous_events()
            .when(LenderGroupCommand::AddLender(Lender { id: 1.to_string(), name: "ABC Bank".to_string(), lender_group_id: 1.to_string() }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_remove_lender() {
        let expected = LenderGroupEvent::LenderRemoved(Lender { id: 2.to_string(), name: "XYZ Credit Union".to_string(), lender_group_id: 1.to_string() });

        LenderGroupTestFramework::default()
            .given_no_previous_events()
            .when(LenderGroupCommand::RemoveLender(Lender { id: 2.to_string(), name: "XYZ Credit Union".to_string(), lender_group_id: 1.to_string() }))
            .then_expect_events(vec![expected]);
    }
}
