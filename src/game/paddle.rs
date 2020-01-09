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

use crate::{FOREHEAD, BOARD_HEIGHT, BOARD_WIDTH};

// the default paddle dimensions
const PADDLE_WIDTH: f32 = 65f32;
const PADDLE_HEIGHT: f32 = 20f32;

// the paddle's y position
const PADDLE_Y: f32 = (BOARD_HEIGHT * 0.9 + FOREHEAD) - PADDLE_HEIGHT / 2.0;

pub struct Paddle
{
    rect: Rect,
    mesh: Mesh,
}

impl Paddle
{
    pub fn new(ctx: &mut Context) -> GameResult<Paddle>
    {
        // the paddle starts in the middle of the board
        let rect = Rect::new(
            BOARD_WIDTH / 2.0 - PADDLE_WIDTH / 2.0,
            PADDLE_Y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT);

        Ok(Paddle
        {
            rect: rect,
            mesh: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                // the mesh is at (0, 0), because we draw it with the rect
                // coords so that it matches correctly
                Rect::new(0f32, 0f32, PADDLE_WIDTH, PADDLE_HEIGHT),
                graphics::BLACK)?,
        })
    }

    pub fn reset(&mut self)
    {
        self.rect = Rect::new(
            BOARD_WIDTH / 2.0 - PADDLE_WIDTH / 2.0,
            PADDLE_Y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT);
    }

    pub fn shift(&mut self, dx: f32)
    {
        self.rect.translate([dx, 0f32]);

        // keep it at most half off the sides of the screen
        if self.rect.x + self.rect.w / 2.0 < 0f32
        {
            self.rect.move_to([-self.rect.w / 2.0, self.rect.y]);
        }
        else if self.rect.x + self.rect.w / 2.0 > BOARD_WIDTH
        {
            self.rect.move_to([BOARD_WIDTH - self.rect.w / 2.0, self.rect.y]);
        }
    }

    pub fn rect(&self) -> Rect
    {
        self.rect
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(ctx, &self.mesh, DrawParam::default()
            // draw the mesh where the paddle rect is
            .dest([self.rect.x, self.rect.y]))
    }
}
