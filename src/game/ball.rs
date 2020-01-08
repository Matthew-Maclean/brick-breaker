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
use super::bricks::Bricks;
use super::utils;

pub const BALL_SIZE: f32 = 5f32;
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

    // maybe there's a better way to do this
    pub fn update(&mut self, paddle: &Paddle, bricks: &mut Bricks) -> UpdateReturn
    {
        let mut destroyed_brick = false;

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
            return UpdateReturn
            {
                destroyed_ball: true,
                destroyed_brick: destroyed_brick,
            };
        }

        match utils::intersect_rect(self.pos, proj, self.size, paddle.rect())
        {
            Some(utils::Axis::X) =>
            {
                let r = paddle.rect();

                // if the projected collision is on the left quarter or right quarter,
                // bounce the ball on an angle
                if proj[0] < r.x + r.w * 0.25
                {
                    self.dir = utils::bounce_angle(self.dir, [10.0, -2.0]);
                }
                else if proj[0] > r.x + r.w * 0.75
                {
                    self.dir = utils::bounce_angle(self.dir, [10.0, 2.0]);
                }
                else
                {
                    bounce_x_axis = true;
                }
            },
            Some(utils::Axis::Y) => bounce_y_axis = true,
            None => { }
        }

        let mut dist = ::std::f32::MAX;
        let mut collision = None;
        for brick in bricks.bricks().iter_mut() // bricks
        {
            if let Some(b) = brick
            {
                if let Some(axis) = utils::intersect_rect(self.pos, proj, self.size, b.rect)
                {
                    let d = utils::dist_to_rect(self.pos, b.rect);
                    if d < dist
                    {
                        dist = d;
                        collision = Some((axis, brick))
                    }
                }
            }
        }

        if let Some((axis, brick)) = collision
        {
            match axis
            {
                utils::Axis::X => bounce_x_axis = true,
                utils::Axis::Y => bounce_y_axis = true,
            }

            if let Some(b) = brick
            {
                b.count -= 1;
                if b.count == 0
                {
                    *brick = None;
                    destroyed_brick = true;
                }
            }
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

        // if the ball has found itself inside the paddle, get it out of there!
        if let Some(pos) = utils::inside_rect(self.pos, self.size, paddle.rect())
        {
            self.pos = pos;
        }

        return UpdateReturn
        {
            destroyed_ball: false,
            destroyed_brick: destroyed_brick,
        }
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

pub struct UpdateReturn
{
    pub destroyed_ball: bool,
    pub destroyed_brick: bool,
}
