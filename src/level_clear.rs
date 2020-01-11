use ggez::
{
    Context,
    GameResult,
    input::mouse::MouseButton,
};

use crate::{BOARD_WIDTH, MAX_LEVEL};
use crate::
{
    state::State,
    menu::Menu,
    level_select::LevelSelect,
    game::Game,
    ui::{TextRect, Button},
};

const BORDER: f32 = 25.0;
const BUTTON_BORDER: f32 = 15.0;
const SIZE: f32 = 35.0;

pub struct LevelClear
{
    title: TextRect,
    info: TextRect,

    next: Option<Button>,
    menu: Button,

    time: u32,
    level: u32,
}

impl LevelClear
{
    pub fn new(ctx: &mut Context, time: u32, level: u32)
        -> GameResult<LevelClear>
    {
        let title = TextRect::new(ctx, 0.0, BORDER, "You Win!", None, SIZE, None)
            .center_x(BOARD_WIDTH);

        let info = TextRect::new(ctx, 0.0, title.rect().bottom() + BORDER,
            format!("time: {}", time),
            None, SIZE * 0.75, None)
            .center_x(BOARD_WIDTH);

        let next = if level < MAX_LEVEL
        {
            let next_text = TextRect::new(ctx, 0.0, info.rect().bottom() + BORDER,
                format!("Next Level (Level {})", level + 1),
                None, SIZE, None)
                .center_x(BOARD_WIDTH);

            Some(Button::new(ctx, next_text, BUTTON_BORDER)?)
        }
        else
        {
            None
        };

        let menu_text = TextRect::new(ctx, 0.0,
            if let Some(ref next) = &next
            {
                next.rect().bottom() + BORDER
            }
            else
            {
                info.rect().bottom() + BORDER
            },
            "Main Menu",
            None, SIZE, None)
            .center_x(BOARD_WIDTH);

        let menu = Button::new(ctx, menu_text, BUTTON_BORDER)?;

        Ok(LevelClear
        {
            title: title,
            info: info,

            next: next,
            menu: menu,

            time: time,
            level: level,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<State>>
    {
        if let Some(ref mut next) = &mut self.next
        {
            if next.click()
            {
                return Ok(Some(State::Game(Game::new(ctx, self.level + 1)?)))
            }
        }
        if self.menu.click()
        {
            return Ok(Some(State::Menu(Menu::new(ctx)?)))
        }
        Ok(None)
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        if let Some(ref mut next) = &mut self.next
        {
            next.mouse_move(x, y);
        }
        self.menu.mouse_move(x, y);
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        if let Some(ref mut next) = &mut self.next
        {
            next.mouse_down(button, x, y);
        }
        self.menu.mouse_down(button, x, y);
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        if let Some(ref mut next) = &mut self.next
        {
            next.mouse_up(button);
        }
        self.menu.mouse_up(button);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        self.title.draw(ctx)?;
        self.info.draw(ctx)?;
        if let Some(ref mut next) = &mut self.next
        {
            next.draw(ctx)?;
        }
        self.menu.draw(ctx)
    }
}
