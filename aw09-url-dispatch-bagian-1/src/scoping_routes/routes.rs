use actix_web::web;

use crate::scoping_routes::handler::{
    show_users,
    user_detail
};

pub fn scoped_scoping_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(show_users)
            .service(user_detail)
    );
}