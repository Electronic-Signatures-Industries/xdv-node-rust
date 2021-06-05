// jsonrpc boilerplate
// https://github.com/paritytech/jsonrpsee/tree/master/examples

mod verifier;
mod chain;

use std::{net::SocketAddr, sync::Arc, time::Duration};
use crate::chain::xdv_chain_node;
use tokio::{sync::Mutex, time::sleep};
use verifier::Verifier;
use verifier::RpcImpl;
use tracing_subscriber::FmtSubscriber;
use tracing::{subscriber::set_global_default, Level};

// use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;


#[tokio::main]
async fn main() -> std::io::Result<()> {
	// let mut io = jsonrpc_core::IoHandler::new();
	// io.extend_with(RpcImpl.to_delegate());

	// let server = ServerBuilder::new(io)
	// 	.threads(3)
	// 	.start_http(&"127.0.0.1:3030".parse().unwrap())
	// 	.unwrap();

	// server.wait();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    set_global_default(subscriber).unwrap();

    let server = xdv_chain_node();
    server
        .run("127.0.0.1:26658".parse::<SocketAddr>().unwrap())
        .await

}
