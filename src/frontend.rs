use poloniex_api_2;

use ggez::*;
use ggez::graphics::{ DrawMode, Point };
use std::time::Duration;

use std::thread;
use std::thread::{
  JoinHandle
};
use std::sync::mpsc::{
  Receiver
};

type Stream = Receiver<poloniex_api_2::State>;

struct State {
  stream: Stream,
  data: poloniex_api_2::State
}

impl State {
  fn new(_ctx: &mut Context, stream: Stream) -> GameResult<State> {
    let s = State {
      stream: stream,
      data: poloniex_api_2::State::new()
    };

    Ok(s)
  }
}

impl event::EventHandler for State {
  fn update(&mut self, _ctx: &mut Context, duration: Duration) -> GameResult<()> {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::circle(ctx, DrawMode::Fill, Point::new(200.0, 200.0), 100.0, 16)?;
    graphics::present(ctx);
    Ok(())
  }
}

pub fn start(stream: Stream) -> JoinHandle<()> {
	thread::spawn(move || {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("poloniex_analyzer", "ggez", c).unwrap();
    let state = &mut State::new(ctx, stream).unwrap();
    event::run(ctx, state).unwrap();
	})
}
