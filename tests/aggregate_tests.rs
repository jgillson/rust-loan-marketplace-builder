#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use rust_loan_marketplace_builder::aggregate::{LenderGroup, Lender};
    use rust_loan_marketplace_builder::commands::{LenderGroupCommand, AddLenderGroup, AddLender, RemoveLender};
    use rust_loan_marketplace_builder::events::{LenderGroupEvent, LenderGroupAdded, LenderAdded, LenderRemoved};

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
