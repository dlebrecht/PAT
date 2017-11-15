use serde_json;

use websocket::{
  ClientBuilder,
  OwnedMessage,
  Message,
  WebSocketError
};

use websocket::sync::Client;
use websocket::sync::stream::NetworkStream;
type WebSocket = Client<Box<NetworkStream + Send>>;

use serde_json::Value;
use serde_json::Value::{ 
  Number
};

use poloniex_api_2::{ 
	State,
	MarketEvent,
	TickerEvent,
  Channel
};

use std::collections::HashMap;
use std::sync::mpsc::{
	Sender
};

use std::thread;
use std::thread::{
  JoinHandle
};

pub fn start(url: String, sink: Sender<State>) -> JoinHandle<()> {
	thread::spawn(move || {
		ClientBuilder::new(url.as_str())
			.map_err(|a| WebSocketError::UrlError(a))
			.and_then(|mut a| a.connect(None))
			.and_then(|mut a| handle_events(&mut a, sink))
			.or_else(handle_error);
	})
}

fn subscribe(client: &mut WebSocket, channel: &str) {
  let json = format!("{{ \"command\": \"subscribe\", \"channel\": {} }}", channel);
  client.send_message(&Message::text(json));
}

const PAIR: &str = "BTC_XMR";

fn handle_events(client: &mut WebSocket, sink: Sender<State>) -> Result<(), WebSocketError> {
  subscribe(client, &(Channel::Ticker as u64).to_string());
  subscribe(client, &(Channel::Stats as u64).to_string());
  subscribe(client, &format!("\"{}\"", PAIR));

  let mut state = State::new();

  for message in client.incoming_messages() {
    message
      .and_then(|msg| handle_message(msg, &mut state, &sink))
      .or_else(handle_error);
  }

  Ok(())
}

fn handle_message(msg: OwnedMessage, state: &mut State, sink: &Sender<State>) -> Result<(), WebSocketError> {
  match msg {
    OwnedMessage::Text(txt) => handle_text(&txt, state, sink),
    _ => ()
  };

  Ok(())
}

fn handle_text(txt: &str, state: &mut State, sink: &Sender<State>) {
  match serde_json::from_str(txt) {
    Ok::<Value, _>(json) => interpret(&json, state, sink),
    Err(error) => { println!("{:?}", error); () }
  }
}

// Where you'd do all your logic.
fn interpret(json: &Value, state: &mut State, sink: &Sender<State>) {
  match get_channel(&json[0]) {
    Channel::MarketXMR => {
      let market_id = serde_json::from_value::<u32>(json[0].clone()).unwrap().to_string();
      MarketEvent::handle(&market_id, json, state);
      sink.send(state.clone()).unwrap();
    },
    Channel::Ticker => { 
      if json[1] == 1 { return; }
      let market_id = serde_json::from_value::<u32>(json[2][0].clone()).unwrap().to_string();
      if market_id != "114" { return; }

      TickerEvent::handle(&market_id, json, state);
      sink.send(state.clone()).unwrap();
    },
    //Channel::Stats => json[2][1].to_string().render(),
    _ => ()
  }
}

fn get_channel(json: &Value) -> Channel {
  let channel_num = match json.clone() {
    Number(num) => num.as_u64(),
    _ => None
  };

  match channel_num {
    Some(n) => Channel::from(n),
    None => Channel::Unknown
  }
}

fn handle_error(error: WebSocketError) -> Result<(), WebSocketError> {
  println!("{:?}", error);
  Err(error)
}

