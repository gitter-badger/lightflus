use std::fs;
use std::sync;
use actix::Actor;

use dataflow_api::dataflow_worker_grpc;
use proto::worker::worker_grpc;

mod api;
pub mod worker;
pub mod actor;
mod constants;

fn main() {
    let result = fs::File::open("src/worker/etc/worker.json");
    if result.is_err() {
        panic!("{}", format!("config file open failed: {:?}", result.unwrap_err()))
    }

    let file = result.unwrap();

    let env_setup = common::sysenv::serde_env::from_reader(file);
    if env_setup.is_err() {
        panic!("{}", format!("config file read failed: {:?}", env_setup.unwrap_err()))
    }

    let value = env_setup.unwrap();
    let reader = serde_json::from_str::<worker::TaskWorkerConfig>(value.as_str());

    if reader.is_err() {
        panic!("{}", format!("config file parse failed: {:?}", reader.unwrap_err()))
    }

    let ref mut config = reader.unwrap();
    let runner = actix::System::new();
    let task_worker = worker::new_worker();

    let server = api::TaskWorkerApiImpl::new(task_worker);
    let service = worker_grpc::create_task_worker_api(server);
    println!("start service at port {}", &config.port);

    let grpc_server = grpcio::ServerBuilder::new(
        sync::Arc::new(grpcio::Environment::new(10)))
        .register_service(service)
        .bind("0.0.0.0", config.port as u16)
        .build();

    if grpc_server.is_err() {
        panic!("{:?}", grpc_server.unwrap_err())
    }

    let mut unwraped_server = grpc_server.unwrap();
    unwraped_server.start();
    runner.run();
    let _ = futures_executor::block_on(unwraped_server.shutdown());

    actix::System::current().stop();
}
