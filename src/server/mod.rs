use std::time::Duration;

use tonic::{transport::Server, Request, Response, Status};
use tracing::{debug, info};

use crate::{
    error,
    grpc_server_rs_template_proto::{
        self,
        grpc_server_rs_template_server::{GrpcServerRsTemplate, GrpcServerRsTemplateServer},
        HealthCheckRequest, HealthCheckResponse,
    },
    server::middleware::check_auth,
};

pub mod middleware;

pub struct ServiceGrpcServerRsTemplate {
    _grpc: String,
}

impl Default for ServiceGrpcServerRsTemplate {
    fn default() -> Self {
        Self {
            _grpc: "_".to_string(),
        }
    }
}

impl ServiceGrpcServerRsTemplate {
    fn new() -> Self {
        Default::default()
    }
}

#[tonic::async_trait]
impl GrpcServerRsTemplate for ServiceGrpcServerRsTemplate {
    async fn health_check(
        &self,
        _: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        debug!("FN: health_check - Service to check if server is up");

        let res = HealthCheckResponse { success: true };
        Ok(Response::new(res))
    }
}

pub async fn start() -> error::Result<()> {
    let addr: std::net::SocketAddr = "0.0.0.0:50051".parse()?;
    let grpc_server_rs_template = ServiceGrpcServerRsTemplate::new();

    info!("Starting gRPC server...");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_server_rs_template_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(GrpcServerRsTemplateServer::with_interceptor(
            grpc_server_rs_template,
            check_auth,
        ))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}

pub async fn start_backgroung() -> error::Result<()> {
    let addr: std::net::SocketAddr = "0.0.0.0:50051".parse()?;
    let grpc_server_rs_template = ServiceGrpcServerRsTemplate::new();

    tokio::spawn(async move {
        let server = Server::builder()
            .add_service(GrpcServerRsTemplateServer::with_interceptor(
                grpc_server_rs_template,
                check_auth,
            ))
            .serve(addr)
            .await;
        if let Err(e) = server {
            e.to_string();
        }
    });

    // Wait for the server to be ready (optional)
    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}
