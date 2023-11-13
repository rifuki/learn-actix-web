use diesel::{
    prelude::{
        QueryableByName, Insertable
    },
    mysql::Mysql,
};


#[derive(QueryableByName, serde::Serialize)]
#[diesel(check_for_backend(Mysql))]
pub struct Item {
    #[diesel(sql_type = diesel::sql_types::VarChar)]
    pub id: String,
    #[diesel(sql_type = diesel::sql_types::VarChar)]
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(Mysql))]
pub struct NewItem {
    pub id: String,
    pub name: String
}

#[derive(serde::Deserialize)]
pub struct CreateItemPayload {
    pub name: String
}