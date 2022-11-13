#[macro_use]
extern crate log;

mod api;
mod repository;
pub mod model;

use api::task::{
  get_task,
  submit_task,
  start_task,
  complete_task,
  pause_task,
  fail_task,
};
use aws_config::profile::{ProfileFileCredentialsProvider};
use aws_config::profile::profile_file::{ProfileFiles, ProfileFileKind};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "debug");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();
  let profile_files = ProfileFiles::builder()
  .with_file(ProfileFileKind::Credentials, "home/timegated/.aws/credentials").build();
  let credentials_provider = ProfileFileCredentialsProvider::builder()
  .profile_files(profile_files.clone())
  .build();
  let config = aws_config::from_env().credentials_provider(credentials_provider).region("us-east-1").load().await;
  println!("{:?}", config);
  // HTTP Server Struct
  HttpServer::new(move || {
    let logger = Logger::default();
    let ddb_repo: DDBRepository = DDBRepository::init(
      String::from("task"),
      config.clone(),
    );
    let ddb_data = Data::new(ddb_repo);
    info!("Starting Service");
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
