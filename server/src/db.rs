#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);
