use ggez::
{
    Context,
    GameResult,
    graphics::
    {
        self,
        Rect,
        Mesh,
        Color,
        DrawMode,
        DrawParam,
    },
    input::mouse::MouseButton,
};

use crate::{FOREHEAD, BOARD_WIDTH};
use crate::ui::{TextRect, Button};

pub struct PauseUI
{
    title: TextRect,
    resume: Button,
    restart: Button,
    main_menu: Button,
    background: Mesh,
}

const BORDER: f32 = 5.0;
const SEP: f32 = 20.0;

impl PauseUI
{
    pub fn new(ctx: &mut Context) -> GameResult<PauseUI>
    {
        let title = TextRect::new(ctx, 0.0, 50.0 + FOREHEAD, "Paused", None, 35.0, None)
            .center_x(BOARD_WIDTH);
        let bg_rect = title.rect();

        let resume_text = TextRect::new(
            ctx,
            0.0,
            title.rect().bottom() + SEP,
            "Resume",
            None,
            25.0,
            None)
            .center_x(BOARD_WIDTH);
        let resume = Button::new(ctx, resume_text, BORDER)?;

        let bg_rect = bg_rect.combine_with(resume.rect());

        let restart_text = TextRect::new(
            ctx,
            0.0,
            resume.rect().bottom() + SEP,
            "Restart",
            None,
            25.0,
            None)
            .center_x(BOARD_WIDTH);
        let restart = Button::new(ctx, restart_text, BORDER)?;

        let bg_rect = bg_rect.combine_with(restart.rect());

        let main_menu_text = TextRect::new(
            ctx,
            0.0,
            restart.rect().bottom() + SEP,
            "Main Menu",
            None,
            25.0,
            None)
            .center_x(BOARD_WIDTH);
        let main_menu = Button::new(ctx, main_menu_text, BORDER)?;

        let bg_rect = bg_rect.combine_with(main_menu.rect());

        let bg_rect = Rect::new(
            bg_rect.x - BORDER,
            bg_rect.y - BORDER,
            bg_rect.w + BORDER * 2.0,
            bg_rect.h + BORDER * 2.0);

        Ok(PauseUI
        {
            title: title,
            resume: resume,
            restart: restart,
            main_menu: main_menu,
            background: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                bg_rect,
                Color::from_rgb(255, 255, 255))?,
        })
    }

    pub fn reset(&mut self)
    {
        self.resume.reset();
        self.restart.reset();
        self.main_menu.reset();
    }

    pub fn resume_click(&self) -> bool
    {
        self.resume.click()
    }

    pub fn restart_click(&self) -> bool
    {
        self.restart.click()
    }

    pub fn main_menu_click(&self) -> bool
    {
        self.main_menu.click()
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        self.resume.mouse_move(x, y);
        self.restart.mouse_move(x, y);
        self.main_menu.mouse_move(x, y);
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        self.resume.mouse_down(button, x, y);
        self.restart.mouse_down(button, x, y);
        self.main_menu.mouse_down(button, x, y);
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        self.resume.mouse_up(button);
        self.restart.mouse_up(button);
        self.main_menu.mouse_up(button);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(ctx, &self.background, DrawParam::new())?;
        self.main_menu.draw(ctx)?;
        self.restart.draw(ctx)?;
        self.resume.draw(ctx)?;
        self.title.draw(ctx)
    }
}
