use jsonrpc_core::Result;
use jsonrpc_derive::{rpc};

#[rpc]
pub trait Verifier {
    /// Adds two numbers and returns a result
    #[rpc(name = "add")]
    fn add(&self, a1: u64, a2: u64) -> Result<u64>;
}


pub struct RpcImpl;
impl Verifier for RpcImpl {
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }
}

