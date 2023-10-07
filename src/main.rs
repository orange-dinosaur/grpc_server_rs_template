use grpc_server_rs_template::{error::Result, run};

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
