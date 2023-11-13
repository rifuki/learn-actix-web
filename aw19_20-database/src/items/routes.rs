use actix_web::{web, guard};

use crate::items::handlers::{
    get_all_items,
    get_single_item,
    create_item,
    update_item,
    delete_item
};

pub fn scoped_items(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            /* * get_all_items */
            .route(
                "",
                web::get()
                    .to(get_all_items)
            )
            /* * get_item */
            .route(
                "/{id}", 
                web::get()
                    .to(get_single_item)
            )
            /* * create_item */
            .route(
                "",
                web::post()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .to(create_item)
            )
            /* * delete_item */
            .route(
                "/{id}",
                web::delete()
                    .to(delete_item)
            )
            /* * update_item */
            .route(
                "/{id}",
                web::route()
                    .guard(guard::Header(
                        "Content-Type", "application/json"
                    ))
                    .to(update_item)
            )
    );
}
