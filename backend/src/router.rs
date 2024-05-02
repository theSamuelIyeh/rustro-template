use crate::routes;
use actix_web::web::ServiceConfig;
use actix_files;

pub fn init_router(cfg: &mut ServiceConfig) {
    cfg.service(routes::hello_world)
        .service(actix_files::Files::new("/", "templates").show_files_listing());
}
