use postgres_es::{Connection, GenericQueryRepository, PostgresCqrs};

use crate::aggregate::LenderGroup;
use crate::queries::{LenderGroupQuery, SimpleLoggingQueryProcessor};

pub(crate) type QueryRepository = GenericQueryRepository::<LenderGroupQuery, LenderGroup>;

pub(crate) fn cqrs_framework() -> PostgresCqrs<LenderGroup> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut query_processor =
        QueryRepository::new("lender_group_query", db_connection());
    query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_query), Box::new(query_processor)])
}

pub(crate) fn db_connection() -> Connection {
    Connection::new("postgresql://test_user:test_pass@localhost:5432/lender_groups")
}
