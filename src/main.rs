extern crate ggez;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate websocket;
#[macro_use] extern crate lazy_static;

mod frontend;
mod backend;
mod poloniex_api_2;

use std::sync::mpsc::{
	channel
};

use poloniex_api_2::{
  URL
};

fn main() {
	let (sink, stream) = channel();
	let frontend = frontend::start(stream);
	let backend = backend::start(URL.to_string(), sink);
	frontend.join().and(backend.join()).or::<()>(Ok(())).unwrap();
}
