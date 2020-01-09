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

const BORDER: f32 = 5.0;
const SEP: f32 = 1.5;

pub struct ForeHead
{
    bg: Mesh,

    score_text: TextRect,
    score_val: TextRect,
    timer_text: TextRect,
    timer_val: TextRect,
    
    ball_r: f32,
    ball: Mesh,

    lives: u32,
    max_score: u32,
}

impl ForeHead
{
    pub fn new(ctx: &mut Context, max_score: u32, starting_lives: u32) -> GameResult<ForeHead>
    {
        let size = (FOREHEAD - BORDER * 2.0 - SEP) / 2.0;
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
            score_text.rect().right() + SEP,
            BORDER,
            format!("000/{:03}", max_score),
            None,
            size,
            Some(graphics::WHITE));
        let timer_text = TextRect::new(
            ctx,
            BORDER,
            score_text.rect().bottom() + SEP,
            "Time:",
            None,
            size,
            Some(graphics::WHITE));
        let timer_val = TextRect::new(
            ctx,
            timer_text.rect().right() + SEP,
            timer_text.rect().top(),
            "0000",
            None,
            size,
            Some(graphics::WHITE));
        
        let ball_r =
            ((FOREHEAD - BORDER * 2.0 - (starting_lives as f32- 1.0) * SEP) / starting_lives as f32)
            / 2.0;

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
            ball_r,
            ball: Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [0.0, 0.0],
                ball_r,
                0.01,
                graphics::WHITE)?,
            
            lives: starting_lives,
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

    pub fn set_lives(&mut self, lives: u32)
    {
        self.lives = lives;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(ctx, &self.bg, DrawParam::new())?;

        self.score_text.draw(ctx)?;
        self.score_val.draw(ctx)?;
        
        self.timer_text.draw(ctx)?;
        self.timer_val.draw(ctx)?;

        for i in 0..self.lives
        {
            graphics::draw(ctx, &self.ball, DrawParam::new()
                .dest([
                    BOARD_WIDTH - BORDER - self.ball_r,
                    BORDER + self.ball_r + i as f32 * self.ball_r * 2.0 + (i as f32 - 1.0) * SEP,
                ]))?;
        }

        Ok(())
    }
}
