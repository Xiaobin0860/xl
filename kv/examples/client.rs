use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CmdReq, CmdRes};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    info!("Connect to {}", addr);

    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    // 使用 AsyncProstStream 来处理 TCP Frame
    let mut client = AsyncProstStream::<_, CmdRes, CmdReq, _>::from(stream).for_async();

    // 生成一个 HSET 命令
    let cmd = CmdReq::new_hset("table1", "hello", "world".to_string().into());

    // 发送 HSET 命令
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    Ok(())
}
