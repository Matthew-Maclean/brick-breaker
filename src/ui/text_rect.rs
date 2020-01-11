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

    font: Font,
    scale: Scale,
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
            font: if let Some(f) = font { f } else { Font::default() },
            scale: Scale::uniform(scale),
        }
    }

    pub fn center_x(mut self, width: f32) -> Self
    {
        let x = width / 2.0 - self.rect.w / 2.0;

        self.rect.move_to([x, self.rect.y]);

        self
    }

    pub fn move_to(&mut self, x: f32, y: f32)
    {
        self.rect.move_to([x, y]);
    }

    pub fn change_text(&mut self, ctx: &mut Context, text: impl ToString)
    {
        let mut t = Text::new(text.to_string());
        t.set_font(self.font, self.scale);
        
        let d = t.dimensions(ctx);

        self.text = t;
        self.rect = Rect::new(self.rect.x, self.rect.y, d.0 as f32, d.1 as f32);
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
