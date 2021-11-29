#[cfg(test)]
mod simple_application_tests {
    use cqrs_es::CqrsFramework;
    use cqrs_es::mem_store::MemStore;

    use crate::aggregate::{LenderGroup, Lender};
    use crate::commands::LenderGroupCommand;
    use crate::queries::SimpleLoggingQueryProcessor;

    #[test]
    fn test_event_store_single_command() {
        let event_store = MemStore::<LenderGroup>::default();
        let query = SimpleLoggingQueryProcessor {};
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)]);

        let aggregate_id = "aggregate-instance-1";

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLender(Lender { id: 1.to_string(), name: "ABC Bank".to_string(), lender_group_id: 1.to_string()})).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLender(Lender { id: 2.to_string(), name: "XYZ Credit Union".to_string(), lender_group_id: 1.to_string()})).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::RemoveLender(Lender { id: 2.to_string(), name: "XYZ Credit Union".to_string(), lender_group_id: 1.to_string()})).unwrap();

        cqrs.execute(aggregate_id, LenderGroupCommand::AddLender(Lender { id: 2.to_string(), name: "XYZ Credit Union".to_string(), lender_group_id: 1.to_string()})).unwrap();
    }
}