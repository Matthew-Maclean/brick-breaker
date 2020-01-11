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
    game::Game,
    ui::{TextRect, Button},
};

const BORDER: f32 = 25.0;
const BUTTON_BORDER: f32 = 15.0;
const SIZE: f32 = 35.0;

const LEVEL_BORDER: f32 = 10.0;
const LEVEL_SIZE: f32 = 25.0;


pub struct LevelSelect
{
    title: TextRect,
    levels: Vec<Button>,
    back: Button,
}

impl LevelSelect
{
    pub fn new(ctx: &mut Context) -> GameResult<LevelSelect>
    {
        let title = TextRect::new(ctx, 0.0, BORDER, "Level Select", None, SIZE, None)
            .center_x(BOARD_WIDTH);

        let (levels, r) = LevelSelect::make_levels(ctx, title.rect().bottom())?;

        let back_text = TextRect::new(ctx,
            0.0, r.bottom() + BORDER,
            "Back", None, SIZE, None)
            .center_x(BOARD_WIDTH);


        let back = Button::new(ctx, back_text, BUTTON_BORDER)?;

        Ok(LevelSelect
        {
            title: title,
            levels: levels,
            back: back,
        })
    }

    fn make_levels(ctx: &mut Context, y: f32) -> GameResult<(Vec<Button>, ggez::graphics::Rect)>
    {
        let mut r = ggez::graphics::Rect::new(0.0, 0.0, 0.0, 0.0);

        let mut v = Vec::new();

        let mut left_w = ::std::f32::MIN;
        let mut right_w = ::std::f32::MIN;

        let mut left = Vec::new();
        let mut right = Vec::new();

        for i in 1..=(MAX_LEVEL / 2)
        {
            let t = TextRect::new(ctx, 0.0, 0.0, format!("Level {}", i), None, LEVEL_SIZE, None);
            let b = Button::new(ctx, t, LEVEL_BORDER)?;

            if b.rect().w > left_w
            {
                left_w = b.rect().w;
            }

            left.push(b);
        }

        for i in (MAX_LEVEL / 2 + 1)..=MAX_LEVEL
        {
            let t = TextRect::new(ctx, 0.0, 0.0, format!("Level {}", i), None, LEVEL_SIZE, None);
            let b = Button::new(ctx, t, LEVEL_BORDER)?;

            if b.rect().w > right_w
            {
                right_w = b.rect().w;
            }

            right.push(b);
        }

        let total_w = left_w + BORDER + right_w;

        let offset = (BOARD_WIDTH - total_w) / 2.0;

        let mut total_h = 0.0;

        for (i, mut b) in left.into_iter().enumerate()
        {
            let inner_offset = (left_w - b.rect().w) / 2.0;

            b.move_to(offset + inner_offset,
                y + total_h + i as f32 * BORDER);
            total_h += b.rect().h;

            v.push(b);
        }

        total_h = 0.0;

        for (i, mut b) in right.into_iter().enumerate()
        {
            let inner_offset = (right_w - b.rect().w) / 2.0;

            b.move_to(offset + left_w + BORDER + inner_offset,
                y + total_h + i as f32 * BORDER);
            total_h += b.rect().h;

            v.push(b);
        }

        for b in v.iter()
        {
            r = r.combine_with(b.rect());
        }

        Ok((v, r))
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<State>>
    {
        for (i, ref b) in self.levels.iter().enumerate()
        {
            if b.click()
            {
                return Ok(Some(State::Game(Game::new(ctx, (i + 1) as u32)?)))
            }
        }

        if self.back.click()
        {
            return Ok(Some(State::Menu(Menu::new(ctx)?)))
        }

        Ok(None)
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        for b in self.levels.iter_mut()
        {
            b.mouse_move(x, y);
        }

        self.back.mouse_move(x, y);
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        for b in self.levels.iter_mut()
        {
            b.mouse_down(button, x, y);
        }

        self.back.mouse_down(button, x, y);
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        for b in self.levels.iter_mut()
        {
            b.mouse_up(button);
        }

        self.back.mouse_up(button);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        self.title.draw(ctx)?;
        for b in self.levels.iter_mut()
        {
            b.draw(ctx)?;
        }
        self.back.draw(ctx)
    }
}
