use diesel::{
    MysqlConnection,
    r2d2::{
        Pool,
        ConnectionManager
    }
};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(database_url: String) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .unwrap_or_else(| err | {
            eprintln!("Err creating pool [{}]", err);
            std::process::exit(1);
        })
}