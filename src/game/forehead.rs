use ggez::
{
    Context,
    GameResult,
    graphics::
    {
        self,
        Rect,
        Mesh,
        DrawMode,
        DrawParam,
    },
};


use crate::{BOARD_WIDTH, FOREHEAD};
use crate::ui::TextRect;

pub struct ForeHead
{
    bg: Mesh,

    score_text: TextRect,
    score_val: TextRect,
    timer_text: TextRect,
    timer_val: TextRect,

    max_score: u32,
}

const BORDER: f32 = 5.0;

impl ForeHead
{
    pub fn new(ctx: &mut Context, max_score: u32) -> GameResult<ForeHead>
    {
        let size = (FOREHEAD - BORDER * 3.0) / 2.0;
        let score_text = TextRect::new(
            ctx,
            BORDER,
            BORDER,
            "Score:",
            None,
            size,
            Some(graphics::WHITE));
        let score_val = TextRect::new(
            ctx,
            score_text.rect().right() + BORDER,
            BORDER,
            format!("000/{:03}", max_score),
            None,
            size,
            Some(graphics::WHITE));
        let timer_text = TextRect::new(
            ctx,
            BORDER,
            score_text.rect().bottom() + BORDER,
            "Time:",
            None,
            size,
            Some(graphics::WHITE));
        let timer_val = TextRect::new(
            ctx,
            timer_text.rect().right() + BORDER,
            timer_text.rect().top(),
            "0000",
            None,
            size,
            Some(graphics::WHITE));
        
        Ok(ForeHead
        {
            bg: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, BOARD_WIDTH, FOREHEAD),
                graphics::BLACK)?,
            score_text: score_text,
            score_val: score_val,
            timer_text: timer_text,
            timer_val: timer_val,

            max_score: max_score,
        })
    }

    pub fn set_score(&mut self, ctx: &mut Context, score: u32)
    {
        self.score_val.change_text(ctx, format!("{:03}/{:03}", score, self.max_score));
    }

    pub fn set_timer(&mut self, ctx: &mut Context, timer: u32)
    {
        self.timer_val.change_text(ctx, format!("{:04}", timer));
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(ctx, &self.bg, DrawParam::new())?;

        self.score_text.draw(ctx)?;
        self.score_val.draw(ctx)?;
        
        self.timer_text.draw(ctx)?;
        self.timer_val.draw(ctx)
    }
}
