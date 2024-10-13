mod db;
use crate::db::DbConnection;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(DbConnection::init())
        .launch()
        .await
        .expect("Couldn't start");
}
