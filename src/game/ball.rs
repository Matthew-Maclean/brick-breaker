use ggez::
{
    Context,
    GameResult,
    graphics::
    {
        self,
        Mesh,
        DrawMode,
        DrawParam,
    },
};

use crate::{BOARD_WIDTH, BOARD_HEIGHT, FOREHEAD};

use super::paddle::Paddle;
use super::utils;

const BALL_SIZE: f32 = 5f32;
const BALL_SPEED: f32 = 2.5f32;

pub struct Ball
{
    size: f32,
    pos: [f32; 2],
    dir: [f32; 2],
    speed: f32,

    mesh: Mesh,
}

impl Ball
{
    pub fn new(ctx: &mut Context, pos: [f32; 2], dir: [f32; 2]) -> GameResult<Ball>
    {
        Ok(Ball
        {
            size: BALL_SIZE,
            pos: pos,
            dir: utils::normalize(dir),
            speed: BALL_SPEED,

            mesh: Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [0f32, 0f32],
                BALL_SIZE,
                0.1,
                graphics::BLACK)?,
        })
    }

    pub fn update(&mut self, paddle: &Paddle) -> bool
    {
        // project position
        let proj =
        [
            self.pos[0] + self.dir[0] * self.speed,
            self.pos[1] + self.dir[1] * self.speed
        ];

        let mut bounce_x_axis = false;
        let mut bounce_y_axis = false;

        // bounce off of the walls
        if proj[0] < self.size || proj[0] + self.size > BOARD_WIDTH
        {
            bounce_y_axis = true;
        }
        if proj[1] < self.size + FOREHEAD
        {
            bounce_x_axis = true;
        }

        if proj[1] - self.size > FOREHEAD + BOARD_HEIGHT
        {
            return false;
        }

        match utils::intersect_rect(self.pos, proj, self.size, paddle.rect())
        {
            Some(utils::Axis::X) => bounce_x_axis = true,
            Some(utils::Axis::Y) => bounce_y_axis = true,
            None => { }
        }

        if bounce_x_axis
        {
            self.dir = utils::bounce_axis(self.dir, utils::Axis::X);
        }
        if bounce_y_axis
        {
            self.dir = utils::bounce_axis(self.dir, utils::Axis::Y);
        }

        self.pos =
        [
            self.pos[0] + self.dir[0] * self.speed,
            self.pos[1] + self.dir[1] * self.speed
        ];

        true
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::draw(
            ctx,
            &self.mesh,
            DrawParam::new()
                .dest(self.pos))
    }
}
