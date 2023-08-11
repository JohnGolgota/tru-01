// use ggez::audio;
// use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
// use oorandom::Rand32;

use ggez::input::keyboard::KeyInput;
// use std::env;
// use std::path;
struct MainState {
    screen: graphics::ScreenImage,
}
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let screen =
            graphics::ScreenImage::new(_ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);
        let s = MainState { screen };
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_screen_image(_ctx, &mut self.screen, Color::BLACK);
        let rect = graphics::Rect::new(10.0, 10.0, 300.0, 150.0);
        let rect_mesh =
            graphics::Mesh::new_rectangle(_ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;
        let draw_param = graphics::DrawParam::new();
        canvas.draw(&rect_mesh, draw_param);
        Ok(())
    }
}
fn main() -> GameResult {
    let cb = ContextBuilder::new("PONG", "YO MERO")
        .window_setup(conf::WindowSetup::default().title("PONG!"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0));

    let (mut ctx, events_loop) = cb.build()?;

    let game = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, game)
}
