use ggez::
{
    Context,
    GameResult,
    graphics::
    {
        self,
        Text,
        Rect,
        Font,
        Scale,
        Color,
        DrawParam,
    }
};

pub struct TextRect
{
    text: Text,
    rect: Rect,
    color: Color,
}

impl TextRect
{
    pub fn new(
        ctx: &mut Context,
        x: f32,
        y: f32,
        text: impl ToString,
        font: Option<Font>,
        scale: f32,
        color: Option<Color>)
        -> TextRect
    {
        let mut t = Text::new(text.to_string());
        t.set_font(if let Some(f) = font { f } else { Font::default() }, Scale::uniform(scale));

        let d = t.dimensions(ctx);

        TextRect
        {
            text: t,
            rect: Rect::new(x, y, d.0 as f32, d.1 as f32),
            color: if let Some(c) = color { c } else { graphics::BLACK },
        }
    }

    pub fn center_x(mut self, width: f32) -> Self
    {
        let x = width / 2.0 - self.rect.w / 2.0;

        self.rect.move_to([x, self.rect.y]);

        self
    }

    pub fn rect(&self) -> Rect
    {
        self.rect
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(ctx, &self.text, DrawParam::new()
            .dest(self.rect.point())
            .color(self.color))       
    }
}
