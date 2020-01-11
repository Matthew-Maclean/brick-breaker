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
    level_select::LevelSelect,
    ui::{TextRect, Button},
};

const BORDER: f32 = 25.0;
const BUTTON_BORDER: f32 = 15.0;
const SIZE: f32 = 35.0;

pub struct Menu
{
    title: TextRect,
    level_select: Button,
    exit: Button,
}

impl Menu
{
    pub fn new(ctx: &mut Context) -> GameResult<Menu>
    {
        let title = TextRect::new(ctx, 0.0, BORDER, "Brick Breaker", None, 35.0, None)
            .center_x(BOARD_WIDTH);

        let level_select_text = TextRect::new(
            ctx,
            0.0,
            title.rect().bottom() + BORDER,
            "Level Select",
            None,
            SIZE,
            None)
            .center_x(BOARD_WIDTH);

        let level_select = Button::new(ctx, level_select_text, BUTTON_BORDER)?;

        let exit_text = TextRect::new(
            ctx,
            0.0,
            level_select.rect().bottom() + BORDER,
            "Exit",
            None,
            SIZE,
            None)
            .center_x(BOARD_WIDTH);

        let exit = Button::new(ctx, exit_text, BUTTON_BORDER)?;

        Ok(Menu
        {
            title: title,
            level_select: level_select,
            exit: exit,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<State>>
    {
        if self.level_select.click()
        {
            return Ok(Some(State::LevelSelect(LevelSelect::new(ctx)?)))
        }
        if self.exit.click()
        {
            ggez::event::quit(ctx)
        }

        Ok(None)
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        self.level_select.mouse_move(x, y);
        self.exit.mouse_move(x, y);
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        self.level_select.mouse_down(button, x, y);
        self.exit.mouse_down(button, x, y);
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        self.level_select.mouse_up(button);
        self.exit.mouse_up(button);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        self.title.draw(ctx)?;
        self.level_select.draw(ctx)?;
        self.exit.draw(ctx)
    }
}
