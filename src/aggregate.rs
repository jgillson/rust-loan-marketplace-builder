use cqrs_es::{Aggregate, AggregateError};
use serde::{Deserialize, Serialize};

use crate::commands::LenderGroupCommand;
use crate::events::{LenderGroupEvent, LenderGroupAdded, LenderAdded, LenderRemoved};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LenderGroup {
    lender_group_id: String,
    lender_group_name: String,
    lenders: Vec<Lender>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lender {
    pub id: String,
    pub name: String,
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

impl Default for LenderGroup {
    fn default() -> Self {
        LenderGroup {
            lender_group_id: "".to_string(),
            lender_group_name: "".to_string(),
            lenders: Default::default(),
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use crate::aggregate::{LenderGroup, Lender};
    use crate::commands::{LenderGroupCommand, AddLenderGroup, AddLender, RemoveLender};
    use crate::events::{LenderGroupEvent, LenderGroupAdded, LenderAdded, LenderRemoved};

    type LenderGroupTestFramework = TestFramework<LenderGroup>;

    #[test]
    fn test_add_lender_group() {
        let expected = LenderGroupEvent::LenderGroupAdded(LenderGroupAdded { lender_group_id: 1.to_string(), lender_group_name: "Banks".to_string() });

        LenderGroupTestFramework::default()
            .given_no_previous_events()
            .when(LenderGroupCommand::AddLenderGroup(AddLenderGroup { lender_group_id: 1.to_string(), lender_group_name: "Banks".to_string() }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_add_lender() {
        let expected = LenderGroupEvent::LenderAdded(LenderAdded { lender: Lender { id: 1.to_string(), name: "ABC Bank".to_string() }});

        LenderGroupTestFramework::default()
            .given_no_previous_events()
            .when(LenderGroupCommand::AddLender(AddLender { lender: Lender { id: 1.to_string(), name: "ABC Bank".to_string() }}))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_remove_lender() {
        let expected = LenderGroupEvent::LenderRemoved(LenderRemoved { lender: Lender { id: 2.to_string(), name: "XYZ Bank".to_string() }});

        LenderGroupTestFramework::default()
            .given_no_previous_events()
            .when(LenderGroupCommand::RemoveLender(RemoveLender { lender: Lender { id: 2.to_string(), name: "XYZ Bank".to_string() }}))
            .then_expect_events(vec![expected]);
    }
}
