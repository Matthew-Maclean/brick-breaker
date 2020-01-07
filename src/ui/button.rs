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

use super::TextRect;

pub struct Button
{
    text: TextRect,
    button: Mesh,
    rect: Rect,

    hover: bool,
    click: bool,
}

impl Button
{
    pub fn new(ctx: &mut Context, text: TextRect, border: f32) -> GameResult<Button>
    {
        let text_rect = text.rect();

        let rect = Rect::new(
            text_rect.x - border,
            text_rect.y - border,
            text_rect.w + border * 2.0,
            text_rect.h + border * 2.0);

        let button = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            rect,
            graphics::WHITE)?;

        Ok(Button
        {
            text: text,
            button: button,
            rect: rect,

            hover: false,
            click: false,
        })
    }

    pub fn rect(&self) -> Rect
    {
        self.rect
    }

    pub fn reset(&mut self)
    {
        self.hover = false;
        self.click = false;
    }

    pub fn click(&self) -> bool
    {
        self.click
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        if self.rect.contains([x, y])
        {
            self.hover = true;
        }
        else
        {
            self.hover = false;
        }
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        if self.rect.contains([x, y]) && button == MouseButton::Left
        {
            self.click = true;
        }
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        if button == MouseButton::Left
        {
            self.click = false;
        }
    }


    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(ctx, &self.button, DrawParam::new()
            .color(if self.click
            {
                Color::from_rgb(64, 64, 64)
            }
            else if self.hover
            {
                Color::from_rgb(127, 127, 127)
            }
            else
            {
                Color::from_rgb(191, 191, 191)
            }))?;
        self.text.draw(ctx)
    }
}
