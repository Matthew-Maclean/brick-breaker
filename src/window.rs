use ggez::
{
    Context,
    GameResult,
    input::
    {
        keyboard::
        {
            KeyCode,
            KeyMods,
        },
        mouse::MouseButton,
    },
    event::EventHandler,
};

use crate::game::Game;

pub struct Window
{
    // temporary
    game: Game,
}

impl Window
{
    pub fn new(ctx: &mut Context) -> GameResult<Window>
    {
        Ok(Window
        {
            game: Game::new(ctx)?
        })
    }
}

impl EventHandler for Window
{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        self.game.update(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, repeat: bool)
    {
        self.game.key_down(key, repeat)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods)
    {
        self.game.key_up(key)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _: f32, _: f32)
    {
        self.game.mouse_move(x, y)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32)
    {
        self.game.mouse_down(button, x, y)
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _: f32,
        _: f32)
    {
        self.game.mouse_up(button)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        use ggez::graphics;

        graphics::clear(ctx, graphics::WHITE);

        self.game.draw(ctx)?;

        graphics::present(ctx)
    }
}
