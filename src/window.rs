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

use crate::state::State;
use crate::menu::Menu;

pub struct Window
{
    state: State,
}

impl Window
{
    pub fn new(ctx: &mut Context) -> GameResult<Window>
    {
        Ok(Window
        {
            state: State::Menu(Menu::new(ctx)?),
        })
    }
}

impl EventHandler for Window
{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        match &mut self.state
        {
            State::Menu(ref mut menu) => if let Some(state) = menu.update(ctx)?
            {
                self.state = state;
            },
            State::LevelSelect(ref mut ls) => if let Some(state) = ls.update(ctx)?
            {
                self.state = state;
            },
            State::Game(ref mut game) => if let Some(state) = game.update(ctx)?
            {
                self.state = state;
            },
            State::LevelClear(ref mut lc) => if let Some(state) = lc.update(ctx)?
            {
                self.state = state;
            },
            _ => { },
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, repeat: bool)
    {
        match &mut self.state
        {
            State::Game(ref mut game) => game.key_down(key, repeat),
            _ => { },
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods)
    {
        match &mut self.state
        {
            State::Game(ref mut game) => game.key_up(key),
            _ => { },
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _: f32, _: f32)
    {
        match &mut self.state
        {
            State::Menu(ref mut menu) => menu.mouse_move(x, y),
            State::LevelSelect(ref mut ls) => ls.mouse_move(x, y),
            State::Game(ref mut game) => game.mouse_move(x, y),
            State::LevelClear(ref mut lc) => lc.mouse_move(x, y),
            _ => { },
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32)
    {
        match &mut self.state
        {
            State::Menu(ref mut menu) => menu.mouse_down(button, x, y),
            State::LevelSelect(ref mut ls) => ls.mouse_down(button, x, y),
            State::Game(ref mut game) => game.mouse_down(button, x, y),
            State::LevelClear(ref mut lc) => lc.mouse_down(button, x, y),
            _ => { },
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _: f32,
        _: f32)
    {
        match &mut self.state
        {
            State::Menu(ref mut menu) => menu.mouse_up(button),
            State::LevelSelect(ref mut ls) => ls.mouse_up(button),
            State::Game(ref mut game) => game.mouse_up(button),
            State::LevelClear(ref mut lc) => lc.mouse_up(button),
            _ => { },
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        use ggez::graphics;

        graphics::clear(ctx, graphics::WHITE);

        match &mut self.state
        {
            State::Menu(ref mut menu) => menu.draw(ctx)?,
            State::LevelSelect(ref mut ls) => ls.draw(ctx)?,
            State::Game(ref mut game) => game.draw(ctx)?,
            State::LevelClear(ref mut lc) => lc.draw(ctx)?,
            _ => { },
        }

        graphics::present(ctx)
    }
}
