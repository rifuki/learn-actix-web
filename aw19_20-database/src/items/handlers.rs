use actix_web::{Responder, HttpResponse, web};
use diesel::{RunQueryDsl, OptionalExtension};
use uuid::Uuid;

use crate::{
    DbPool,
    items::models as ItemModel,
    schema::items::dsl::{
        items as ItemTable,
        // id as ItemId,
        // name as ItemName,
    }
};

pub async fn get_all_items(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get connection from pool");

    let sql_query = "SELECT * FROM items";
    let results = diesel::sql_query(sql_query)
        .load::<ItemModel::Item>(&mut conn)
        .expect("error executing select all query")
        .iter()
        .map(|item| ItemModel::Item {
            id: item.id.clone(),
            name: item.name.clone()
        })
        .collect::<Vec<ItemModel::Item>>();

    HttpResponse::Ok()
        .json(results)
}

pub async fn get_single_item(pool: web::Data<DbPool>, path: web::Path<(String,)>) -> impl Responder {
    let item_id = path.into_inner().0;
    let mut conn = pool.get()
        .expect("failed to get connection from pool");

    let sql_query = "SELECT * FROM items WHERE id = ?";

    let result = diesel::sql_query(sql_query)
        .bind::<diesel::sql_types::VarChar, _>(item_id)
        .get_result::<ItemModel::Item>(&mut conn)
        .optional()
        .map(|item| {
            match item {
                Some(item) => HttpResponse::Ok().json(item),
                None => HttpResponse::NotFound().body("item not found")
            }
        })
        .unwrap_or_else(|err| {
            eprintln!("error executing SELECT sql query: {:?}", err);
            HttpResponse::InternalServerError()
                .body(
                    format!("error executing SELECT sql query {:?}", err)
                )
        });

        result

}

pub async fn create_item(pool: web::Data<DbPool>, payload: web::Json<ItemModel::CreateItemPayload>) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("failed to get connection from Pool");

    let new_item = ItemModel::NewItem {
        id: Uuid::new_v4().to_string(),
        name: payload.name.clone()
    };

    let result = diesel::insert_into(ItemTable)
        .values(new_item)
        .execute(&mut conn);

    match result {
        Ok(row) => {
            if row >= 1 {
                HttpResponse::Created()
                    .body("item created successfully")
            } else {
                HttpResponse::BadRequest()
                    .body("error creating item")
            }
        },
        Err(err) => {
            HttpResponse::InternalServerError()
                .body(
                    format!("error executing INSERT query: {}",err)
                )
        }
    }
}

pub async fn update_item(
    pool: web::Data<DbPool>, 
    path: web::Path<(String,)>, 
    payload: web::Json<ItemModel::CreateItemPayload>
) -> impl Responder {
    let item_id = path.into_inner().0;

    let mut conn = pool.get().expect("failed to get connection from pool");

    let sql_query = "UPDATE FROM items SET name = ? WHERE id = ?";
    let result = diesel::sql_query(sql_query)
        .bind::<diesel::sql_types::VarChar, _>(payload.name.clone())
        .bind::<diesel::sql_types::VarChar, _>(item_id.clone())
        .execute(&mut conn);

    match result {
        Ok(row) => {
            if row >= 1 {
                HttpResponse::Ok()
                    .body(
                        format!("id: {} updated", item_id)
                    )
            } else {
                HttpResponse::BadRequest()
                    .body("failed to updated data")
            }
        },
        Err(err) => {
            HttpResponse::InternalServerError()
                .body(
                    format!(
                        "error executing UPDATE query: {:?}",
                        err
                    )
                )
        }
    }
}

pub async fn delete_item(pool: web::Data<DbPool>, path: web::Path<(String,)>) -> impl Responder {
    let item_id = path.into_inner().0;
    println!("{}", item_id);

    let mut conn = pool
        .get()
        .expect("failed to get connection from pool");


    let sql_query = "DELETE FROM items WHERE id = ?";
    let result = diesel::sql_query(sql_query)
        .bind::<diesel::sql_types::VarChar, _>(item_id)
        .execute(&mut conn);
    
    match result {
        Ok(row) => {
            if row >= 1 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::BadRequest()
                    .body("item not found")
            }
        },
        Err(err) => {
            HttpResponse::InternalServerError()
                .body(
                    format!("error executing DELETE query: {}", err)
                )
        }
    }
}
