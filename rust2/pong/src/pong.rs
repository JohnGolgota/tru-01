// use ggez;
use ggez::{
    conf,
    conf::FullscreenType,
    event,
    glam::*,
    graphics::{self, Drawable, Mesh},
    input::keyboard::{KeyCode, KeyInput, KeyMods},
    Context, ContextBuilder, GameResult,
};

type Point2 = Vec2;
type Vector2 = Vec2;

const RACKET_HEIGT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGT_HALF: f32 = RACKET_HEIGT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
enum ActorType {
    Player,
    Pelota,
}
struct Actor {
    tag: ActorType,
    pos: Point2,
    mesh: Mesh,
    velocity: Vector2,
    ang_vel: f32,
    facing: f32,
}
fn create_player(mesh: Mesh, pos: Point2) -> Actor {
    Actor {
        tag: ActorType::Player,
        pos,
        mesh,
        velocity: Vector2::new(0.0, 4.0),
        facing: 0.0,
        ang_vel: 0.0,
    }
}
fn create_pelota(mesh: Mesh) -> Actor {
    Actor {
        tag: ActorType::Pelota,
        pos: Point2::ZERO,
        mesh,
        velocity: Vector2::new(8.0, 8.0),
        facing: 0.0,
        ang_vel: 0.0,
    }
}
struct MainState {
    player_1: Actor,
    player_2: Actor,
    ball: Actor,
    screen_width: f32,
    screen_height: f32,
}
impl MainState {
    // fn new(ctx: &mut Context) -> GameResult<MainState> {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: -RACKET_WIDTH_HALF,
                y: -RACKET_HEIGT_HALF,
                w: RACKET_WIDTH,
                h: RACKET_HEIGT,
            },
            graphics::Color::WHITE,
        )?;
        let rectangle_2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: -RACKET_WIDTH_HALF,
                y: -RACKET_HEIGT_HALF,
                w: RACKET_WIDTH,
                h: RACKET_HEIGT,
            },
            graphics::Color::WHITE,
        )?;
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            10.0,
            2.0,
            graphics::Color::MAGENTA,
        )?;
        let (width, height) = ctx.gfx.drawable_size();
        let a: FullscreenType = FullscreenType::Desktop;
        let _ = ctx.gfx.set_fullscreen(a);
        let s = MainState {
            player_1: create_player(rectangle, vec2(-350.0, 0.)),
            // player_1: create_player(rectangle, Point2::X),
            player_2: create_player(rectangle_2, vec2(350.0, 0.)),
            // player_2: create_player(rectangle_2, Point2::NEG_X),
            ball: create_pelota(ball),
            screen_width: width,
            screen_height: height,
        };
        Ok(s)
    }
}
fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}
fn draw_actor(
    drawable: &impl Drawable,
    canvas: &mut graphics::Canvas,
    actor: &Actor,
    world_coords: (f32, f32),
) {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        .rotation(actor.facing)
        .offset(Point2::new(0.5, 0.5));
    canvas.draw(drawable, drawparams)
}
const MAX_PHYSICS_VEL: f32 = 250.0;

fn update_pelota_position(actor: &mut Actor, dt: f32) {
    let norm_sq = actor.velocity.length_squared();
    if norm_sq > MAX_PHYSICS_VEL.powi(2) {
        actor.velocity = actor.velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
    }
    let dv = actor.velocity * dt;
    actor.pos += dv;
    actor.facing += actor.ang_vel;
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let ctx_k = &_ctx.keyboard;

        if ctx_k.is_key_pressed(KeyCode::Up) {
            self.player_1.pos.y += 5.0;
        }
        if ctx_k.is_key_pressed(KeyCode::Down) {
            self.player_1.pos.y -= 5.0;
        }
        update_pelota_position(&mut self.ball, 0.1);
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // let draw_param = graphics::DrawParam::default();
        let player = &self.player_1;
        let coords = (self.screen_width, self.screen_height);
        draw_actor(&self.player_1.mesh, &mut canvas, player, coords);

        let player_2 = &self.player_2;
        draw_actor(&self.player_2.mesh, &mut canvas, player_2, coords);

        let ball = &self.ball;
        draw_actor(&self.ball.mesh, &mut canvas, ball, coords);
        // canvas.draw(&self.ball.mesh, Vec2::new(self.ball.position, 200.0));
        // canvas.draw(&self.rectangle, Vec2::new(self.pos_x, 200.0));
        canvas.finish(_ctx)?;
        Ok(())
    }
}
pub fn main() -> GameResult {
    let cb = ContextBuilder::new("PONG", "YO MERO")
        .window_setup(conf::WindowSetup::default().title("Pong!"));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
