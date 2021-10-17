use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kvs::{CmdReq, CmdRes, MemTable, Service};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream = AsyncProstStream::<_, CmdReq, CmdRes, _>::from(stream).for_async();
            while let Some(Ok(req)) = stream.next().await {
                let res = svc.execute(req);
                stream.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
