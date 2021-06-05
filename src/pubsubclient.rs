use std::sync::{atomic, Arc};
use std::{thread, time};

use jsonrpc_core::*;
use jsonrpc_pubsub::{PubSubHandler, Session, Subscriber, SubscriptionId};
use jsonrpc_tcp_server::{RequestContext, ServerBuilder};


#[rpc]
pub trait Query {
    /// Adds two numbers and returns a result
    #[rpc(name = "add")]
    fn add(&self, a1: u64, a2: u64) -> Result<u64>;
}


pub struct QueryImpl;
impl Query for QueryImpl {
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }
}


/// To test the server:
///
/// ```bash
/// $ netcat localhost 3030 -
/// {"id":1,"jsonrpc":"2.0","method":"hello_subscribe","params":[10]}
///
/// ```
fn main() {
	let mut io = PubSubHandler::new(MetaIoHandler::default());
	io.add_sync_method("say_hello", |_params: Params| Ok(Value::String("hello".to_string())));

	let is_done = Arc::new(atomic::AtomicBool::default());
	let is_done2 = is_done.clone();
	io.add_subscription(
		"hello",
		("subscribe_hello", move |params: Params, _, subscriber: Subscriber| {
			if params != Params::None {
				subscriber
					.reject(Error {
						code: ErrorCode::ParseError,
						message: "Invalid parameters. Subscription rejected.".into(),
						data: None,
					})
					.unwrap();
				return;
			}

			let is_done = is_done.clone();
			thread::spawn(move || {
				let sink = subscriber.assign_id(SubscriptionId::Number(5)).unwrap();
				// or subscriber.reject(Error {} );
				// or drop(subscriber)

				loop {
					if is_done.load(atomic::Ordering::SeqCst) {
						return;
					}

					thread::sleep(time::Duration::from_millis(100));
					match sink.notify(Params::Array(vec![Value::Number(10.into())])) {
						Ok(_) => {}
						Err(_) => {
							println!("Subscription has ended, finishing.");
							break;
						}
					}
				}
			});
		}),
		("remove_hello", move |_id: SubscriptionId, _| {
			println!("Closing subscription");
			is_done2.store(true, atomic::Ordering::SeqCst);
			futures::future::ok(Value::Bool(true))
		}),
	);

}
