mod api;
mod repository;

use api::task:: {
  get_task
};

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "debug");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();
  let config = aws_config::load_from_env().await;
  // HTTP Server Struct
  HttpServer::new(move || {
    let logger = Logger::default();
    let ddb_repo: DDBRepository = DDBRepository::init(
      String::from("task"),
      config.clone(),
    );
    let ddb_data = Data::new(ddb_repo);
    App::new()
    .wrap(logger)
    .app_data(ddb_data)
    .service(get_task)
    .service(submit_task)
    .service(start_task)
    .service(complete_task)
    .service(pause_task)
    .service(fail_task)
  })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
