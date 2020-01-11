use ggez::
{
    Context,
    GameResult,
    input::mouse::MouseButton,
};

use crate::BOARD_WIDTH;
use crate::
{
    state::State,
    menu::Menu,
    game::Game,
    ui::{TextRect, Button},
};

const BORDER: f32 = 25.0;
const BUTTON_BORDER: f32 = 15.0;
const SIZE: f32 = 35.0;

pub struct LevelLose
{
    title: TextRect,
    info: TextRect,

    retry: Button,
    menu: Button,

    level: u32,
}

impl LevelLose
{
    pub fn new(ctx: &mut Context, time: u32, score: u32, max_score: u32, level: u32)
        -> GameResult<LevelLose>
    {
        let title = TextRect::new(ctx, 0.0, BORDER,
            "You Lose!", None, SIZE, None)
            .center_x(BOARD_WIDTH);

        let info = TextRect::new(ctx, 0.0, title.rect().bottom() + BORDER,
            format!("time: {} | score: {}/{}", time, score, max_score),
            None, SIZE * 0.75, None)
            .center_x(BOARD_WIDTH);

        let retry_text = TextRect::new(ctx, 0.0, info.rect().bottom() + BORDER,
            format!("Retry (level {})", level),
            None, SIZE, None)
            .center_x(BOARD_WIDTH);

        let retry = Button::new(ctx, retry_text, BUTTON_BORDER)?;

        let menu_text = TextRect::new(ctx, 0.0, retry.rect().bottom() + BORDER,
            "Main Menu", None, SIZE, None)
            .center_x(BOARD_WIDTH);

        let menu = Button::new(ctx, menu_text, BUTTON_BORDER)?;

        Ok(LevelLose
        {
            title: title,
            info: info,
            retry: retry,
            menu: menu,
            level: level,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<State>>
    {
        if self.retry.click()
        {
            return Ok(Some(State::Game(Game::new(ctx, self.level)?)));
        }
        if self.menu.click()
        {
            return Ok(Some(State::Menu(Menu::new(ctx)?)));
        }

        Ok(None)
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        self.retry.mouse_move(x, y);
        self.menu.mouse_move(x, y);
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        self.retry.mouse_down(button, x, y);
        self.menu.mouse_down(button, x, y);
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        self.retry.mouse_up(button);
        self.menu.mouse_up(button);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        self.title.draw(ctx)?;
        self.info.draw(ctx)?;
        self.retry.draw(ctx)?;
        self.menu.draw(ctx)
    }
}
