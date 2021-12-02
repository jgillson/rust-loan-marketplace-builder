use postgres_es::{Connection, GenericQueryRepository, PostgresCqrs};

use crate::aggregate::LenderGroup;
use crate::queries::{LenderGroupQuery, SimpleLoggingQueryProcessor};

// This provides a simple query repository that can be used both to return deserialized views and to act as a query processor
pub(crate) type QueryRepository = GenericQueryRepository::<LenderGroupQuery, LenderGroup>;

// Create a CqrsFramework backed by PostgresStore and using a simple metadata supplier with time of commit
pub(crate) fn cqrs_framework() -> PostgresCqrs<LenderGroup> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut query_processor =
        QueryRepository::new("lender_group_query", db_connection());
    query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_query), Box::new(query_processor)])
}

// Connect to a Postgres database
pub(crate) fn db_connection() -> Connection {
    Connection::new("postgresql://test_user:test_pass@localhost:5432/lender_groups")
}
