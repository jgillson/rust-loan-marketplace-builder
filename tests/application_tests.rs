#[cfg(test)]
mod simple_application_tests {
    use cqrs_es::CqrsFramework;
    use cqrs_es::mem_store::MemStore;

    use rust_loan_marketplace_builder::aggregate::{LenderGroup, Lender};
    use rust_loan_marketplace_builder::commands::{LenderGroupCommand, AddLender, RemoveLender, AddLenderGroup};
    use rust_loan_marketplace_builder::queries::SimpleLoggingQueryProcessor;

    #[test]
    fn test_event_store_single_command() {
        let event_store = MemStore::<LenderGroup>::default();
        let query = SimpleLoggingQueryProcessor {};
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)]);

        let aggregate_id = "aggregate-instance-1";

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLenderGroup(AddLenderGroup { lender_group_id: 1.to_string(), lender_group_name: "Banks".to_string() })).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLender(AddLender { lender: Lender { id: 1.to_string(), name: "ABC Bank".to_string() }})).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLender(AddLender { lender: Lender { id: 2.to_string(), name: "XYZ Bank".to_string() }})).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::RemoveLender(RemoveLender { lender: Lender { id: 2.to_string(), name: "XYZ Bank".to_string() }})).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLender(AddLender { lender: Lender { id: 2.to_string(), name: "XYZ Bank".to_string() }})).unwrap();
    }
}
