use tonic::transport::Server;
use futures::future;
use warp::Filter;
use log::{
    debug,
    error,
    info,
    warn,
};

mod prometheus;
use crate::prometheus::metrics_handler;

mod vald;
use crate::vald::{
    ValdImpl,
    InsertServer,
    SearchServer,
    AgentServer,
};

mod ngt;
use crate::ngt::NGT;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut vald = ValdImpl::default();

    vald.initialize().unwrap();

    let grpc_server = Server::builder()
        .add_service(InsertServer::new(vald.clone()))
        .add_service(SearchServer::new(vald.clone()))
        .add_service(AgentServer::new(vald))
        .serve(addr);
    info!("gRPC server started: {}", addr);

    let metrics_route = warp::path!("metrics").and_then(metrics_handler);
    let (_metrics_addr, metrics_warp) = warp::serve(metrics_route)
        .bind_ephemeral(([0, 0, 0, 0], 9090));
    info!("Prometheus server started: 0.0.0.0:9090");

    future::join(grpc_server, metrics_warp).await;
}
