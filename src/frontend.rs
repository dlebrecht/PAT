use poloniex_api_2;

use ggez::*;
use ggez::graphics::{
  Color,
  Point,
  Font,
  Text
};

use std::time::Duration;
use std::sync::mpsc::{
  Receiver
};

type Stream = Receiver<poloniex_api_2::State>;

/*
  Composui - A simple component based UI framework.
*/
trait Component {
  fn execute(&mut self, ctx: &mut Context, state: &poloniex_api_2::State) -> GameResult<()>;
  fn get_children(&mut self) -> &mut [Box<Component>];
}

struct Label {
  text: String,
  font: Font,
  rendered_text: Text,
  color: Color,
  position: Point,
  rotation: f32,
  closure: fn(&mut Label, ctx: &mut Context, &poloniex_api_2::State) -> ()
}

impl Component for Label {
  fn execute(&mut self, ctx: &mut Context, state: &poloniex_api_2::State) -> GameResult<()> {
    (self.closure)(self, ctx, state);

    graphics::set_color(ctx, self.color)?;

    let calculated_position = Point {
      x: self.position.x + (self.rendered_text.width() / 2) as f32,
      y: self.position.y
    };
    graphics::draw(ctx, &self.rendered_text, calculated_position, self.rotation)?;

    Ok(())
  }

  fn get_children(&mut self) -> &mut [Box<Component>] {
    &mut []
  }
}

impl Label {
  fn new(ctx: &mut Context, text: &str, font: Font, color: Color, position: Point, rotation: f64, closure: fn(&mut Label, &mut Context, &poloniex_api_2::State) -> ()) -> Box<Label> {
    Box::new(Label {
      text: text.to_string(),
      font: font.clone(),
      rendered_text: Text::new(ctx, text, &font).unwrap(),
      color,
      position,
      rotation: rotation as f32,
      closure
    })
  }

  fn set_text(&mut self, ctx: &mut Context, text: String) -> () {
    if text == self.text { () }

    self.text = text;
    self.rendered_text = Text::new(ctx, self.text.as_str(), &self.font).unwrap();
  }
}

struct UIState {
  stream: Stream,
  data: poloniex_api_2::State,
  update_duration: Duration,
  root: Vec<Box<Component>>
}

impl UIState {
  fn new(ctx: &mut Context, stream: Stream) -> GameResult<UIState> {
    let font = Font::default_font().unwrap();
    let color = graphics::WHITE;//Color { r: 250.0, g: 250.0, b: 250.0, a: 255.0 };

    let s = UIState {
      stream: stream,
      data: poloniex_api_2::State::new(),
      update_duration: Duration::new(1, 0),
      root: vec![
        Label::new(ctx,
          "Monero", font.clone(), color,
          Point { x: 16.0f32, y: 16.0f32 },
          0.0,
          |_, _, _| {}
        ),
        Label::new(ctx,
          "0.00000000", font.clone(), color,
          Point { x: 16.0f32, y: 40.0f32 },
          0.0,
          |this, context, state| {
            state
              .prices
              .get(&114u64) // Need nicer way to reference markets.
              .and_then(|existing| {
                existing.sell.changes.iter().last()
              })
              .and_then(|sell_price| {
                  this.set_text(context, format!("{:.*}", 8, sell_price));
                  Some(())
              });
          }
        ),
        Label::new(ctx,
          "0.00000000", font.clone(), color,
          Point { x: 16.0f32, y: 62.0f32 },
          0.0,
          |this, context, state| {
            state
              .order_books
              .get(&114u64) // Need nicer way to reference markets.
              .and_then(|existing| {
                existing.sell_volume.changes.iter().last()
              })
              .and_then(|sell_volume| {
                  this.set_text(context, format!("{:.*}", 8, sell_volume));
                  Some(())
              });
          }
        ),
        Label::new(ctx,
          "0s", font.clone(), color,
          Point { x: 150.0f32, y: 16.0f32 },
          0.0,
          |this, context, state| {
            state
              .prices
              .get(&114u64) // Need nicer way to reference markets.
              .and_then(|existing| {
                Some(existing
                  .sell
                  .last_updates
                  .iter()
                  .map(|&(a,d)| d.num_seconds() as f64)
                  .sum::<f64>() / existing.sell.last_updates.len() as f64
                )
              })
              .and_then(|avg_update_time:f64| {
                  this.set_text(context, format!("{:.*}s", 2, avg_update_time));
                  Some(())
              });
          }
        )
      ]
    };

    Ok(s)
  }
}

impl event::EventHandler for UIState {
  fn update(&mut self, _ctx: &mut Context, duration: Duration) -> GameResult<()> {
    self.update_duration = self.update_duration + duration;

    if self.update_duration.as_secs() >= 1 {
      self.data = match self.stream.iter().next() {
        Some(state) => state,
        None => self.data.clone()
      };
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    graphics::set_background_color(ctx, Color {
      r: 0.1,
      g: 0.1,
      b: 0.1,
      a: 0.0
    });

    for child in &mut self.root {
      child.execute(ctx, &self.data)?;
    }

    graphics::present(ctx);
    Ok(())
  }
}

pub fn start(stream: Stream) { //-> JoinHandle<()> {
  let mut c = conf::Conf::new();
  c.window_title = "P.A.T.".to_string();
  c.window_width = 200;
  c.window_height = 90;
  c.vsync = true;

  let ctx = &mut Context::load_from_conf("poloniex_analyzer", "ggez", c).unwrap();
  let state = &mut UIState::new(ctx, stream).unwrap();
  event::run(ctx, state).unwrap();
}
