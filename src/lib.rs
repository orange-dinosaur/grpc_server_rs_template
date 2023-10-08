pub mod config;
pub mod error;
pub mod server;
pub mod tracing;

use crate::error::Result;

mod grpc_server_rs_template_proto {
    #![allow(non_snake_case)]
    include!("grpc_server_rs_template.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("grpc_server_rs_template_descriptor");
}

pub async fn run() -> Result<()> {
    // TODO: create custom func to print this
    println!(
        "
 ██████╗ ██████╗ ██████╗  ██████╗    ████████╗███████╗███╗   ███╗██████╗ ██╗      █████╗ ████████╗███████╗
██╔════╝ ██╔══██╗██╔══██╗██╔════╝    ╚══██╔══╝██╔════╝████╗ ████║██╔══██╗██║     ██╔══██╗╚══██╔══╝██╔════╝
██║  ███╗██████╔╝██████╔╝██║            ██║   █████╗  ██╔████╔██║██████╔╝██║     ███████║   ██║   █████╗  
██║   ██║██╔══██╗██╔═══╝ ██║            ██║   ██╔══╝  ██║╚██╔╝██║██╔═══╝ ██║     ██╔══██║   ██║   ██╔══╝  
╚██████╔╝██║  ██║██║     ╚██████╗       ██║   ███████╗██║ ╚═╝ ██║██║     ███████╗██║  ██║   ██║   ███████╗
 ╚═════╝ ╚═╝  ╚═╝╚═╝      ╚═════╝       ╚═╝   ╚══════╝╚═╝     ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝
        "
    );

    // Initialize tracing
    tracing::initialize();

    // start gRPC server
    server::start().await?;

    Ok(())
}
