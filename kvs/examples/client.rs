use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kvs::{CmdReq, CmdRes};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let stream = TcpStream::connect(addr).await?;
    let mut client = AsyncProstStream::<_, CmdRes, CmdReq, _>::from(stream).for_async();

    let cmd = CmdReq::new_hset("table1", "hello", "world");
    client.send(cmd).await?;
    if let Some(res) = client.next().await {
        info!("Get res {:?}", res);
    }

    Ok(())
}
