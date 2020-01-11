use ggez::
{
    ContextBuilder,
    GameResult,
    conf::
    {
        WindowSetup,
        WindowMode,
    },
    event::run,
};

// the width and height of the playable area
const BOARD_WIDTH: f32 = 500f32;
const BOARD_HEIGHT: f32 = 500f32;
// the height of the top information bar
const FOREHEAD: f32 = 50f32;

// the width and height of the whole screen
const WIDTH: f32 = BOARD_WIDTH;
const HEIGHT: f32 = BOARD_HEIGHT + FOREHEAD;

// the highest level
const MAX_LEVEL: u32 = 10;

mod ui;
mod window;
mod state;
mod menu;
mod level_select;
mod game;
mod level_clear;
mod level_lose;

fn main() -> GameResult<()>
{
    let (mut ctx, mut eloop) = ContextBuilder::new("...", "...")
        .window_setup(WindowSetup::default()
            .title("Brick Breaker"))
        .window_mode(WindowMode::default()
            .dimensions(WIDTH, HEIGHT))
        .build()?;

    let mut window = window::Window::new(&mut ctx)?;

    run(&mut ctx, &mut eloop, &mut window)
}
