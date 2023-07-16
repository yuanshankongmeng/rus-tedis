use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use rus_tedis::{CommandRequest, CommandResponse, Kvpair};
use tokio::net::TcpStream;
use tracing::info;

pub struct PanicError;
use std::fmt;
impl<E> From<E> for PanicError
where
    E: fmt::Debug,
{
    fn from(e: E) -> Self {
        panic!("{:?}", e)
    }
}

pub fn unwrap<T>(r: Result<T, PanicError>) -> T {
    if let Ok(t) = r {
        t
    } else {
        unreachable!();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // connet to the server
    let stream = TcpStream::connect(addr).await?;

    // Handle TCP Frame using AsyncProstStream
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    // Create a HSET command
    let cmd = CommandRequest::new_hmset(
        "table1",
        vec![
            Kvpair::new("key1", "value1".into()),
            Kvpair::new("key2", "value2".into()),
        ],
    );

    // Send HSET command
    client.send(cmd.clone()).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    let cmd = CommandRequest::new_hget("table1", "key1");
    client.send(cmd.clone()).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    Ok(())
}
