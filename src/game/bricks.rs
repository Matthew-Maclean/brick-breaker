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
};

use crate::{BOARD_WIDTH, FOREHEAD};

pub const BRICK_WIDTH: f32 = 32.36f32;
pub const BRICK_HEIGHT: f32 = 20f32;

pub struct Bricks
{
    bricks: Vec<Option<Brick>>,
    reset: Vec<Option<Brick>>,
    mesh: Mesh,
}

impl Bricks
{
    pub fn new(ctx: &mut Context, pattern: Vec<((f32, f32), u32)>) -> GameResult<Bricks>
    {
        let bricks = pattern.into_iter()
            .map(|((x, y), c)| { Some(Brick
            {
                rect: Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT),
                count: c,
            })})
            .collect::<Vec<_>>();
        Ok(Bricks
        {
            bricks: bricks.clone(),
            reset: bricks,
            mesh: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0f32, 0f32, BRICK_WIDTH, BRICK_HEIGHT),
                graphics::WHITE)?,
        })
    }

    pub fn total(&self) -> u32
    {
        self.bricks.iter()
            .fold(0, |a, b| { a + if let Some(b) = b { b.count } else { 0 } })
    }

    pub fn bricks(&mut self) -> &mut [Option<Brick>]
    {
        &mut self.bricks
    }

    pub fn reset(&mut self)
    {
        self.bricks = self.reset.clone();
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>
    {
        for brick in &self.bricks
        {
            if let Some(brick) = brick
            {
                graphics::draw(ctx, &self.mesh, DrawParam::new()
                    .dest(brick.rect.point())
                    .color(Bricks::get_color(brick.count)))?;
            }
        }

        Ok(())
    }

    fn get_color(c: u32) -> Color
    {
        match c
        {
            0  => Color::from_rgb(255, 255, 255),
            1  => Color::from_rgb(255, 0  , 0  ),
            2  => Color::from_rgb(255, 64 , 0  ),
            3  => Color::from_rgb(255, 191, 0  ),
            4  => Color::from_rgb(255, 255, 0  ),
            5  => Color::from_rgb(191, 255, 0  ),
            6  => Color::from_rgb(128, 255, 0  ),
            7  => Color::from_rgb(64 , 255, 0  ),
            8  => Color::from_rgb(0  , 255, 0  ),
            9  => Color::from_rgb(0  , 255, 64 ),
            10 => Color::from_rgb(0  , 255, 128),
            11 => Color::from_rgb(0  , 255, 191),
            12 => Color::from_rgb(0  , 255, 255),
            13 => Color::from_rgb(0  , 191, 255),
            14 => Color::from_rgb(0  , 128, 255),
            15 => Color::from_rgb(0  , 64 , 255),
            16 => Color::from_rgb(0  , 0  , 255),
            17 => Color::from_rgb(64 , 0  , 255),
            18 => Color::from_rgb(128, 0  , 255),
            19 => Color::from_rgb(191, 0  , 255),
            20 => Color::from_rgb(255, 0  , 255),
            _  => Color::from_rgb(255, 255, 255),
        }
    }

    pub fn make_pattern(p: Vec<Vec<u32>>) -> Vec<((f32, f32), u32)>
    {
        let mut pattern = Vec::new();

        for (y, row) in p.into_iter().enumerate()
        {
            let offset = (BOARD_WIDTH - row.len() as f32 * BRICK_WIDTH) / 2f32;
            for (x, c) in row.into_iter().enumerate()
            {
                if c > 0
                {
                    pattern.push((
                        (
                            offset + x as f32 * BRICK_WIDTH,
                            y as f32 * BRICK_HEIGHT + FOREHEAD
                        ),
                        c
                    ));
                }
            }
        }

        pattern
    }

    pub fn get_pattern(ctx: &mut Context, level: u32) -> GameResult<Bricks>
    {
        match level
        {
            0  => Bricks::new(ctx, Bricks::make_pattern(Vec::new())),
            1  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
            ])),
            2  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![1, 1, 2, 2, 2, 2, 1, 1],
                vec![1, 1, 2, 3, 3, 2, 1, 1],
                vec![1, 1, 2, 2, 2, 2, 1, 1],
            ])),
            3  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![2, 2, 2, 2, 2, 2, 2, 2],
                vec![2, 3, 3, 4, 4, 3, 3, 2],
                vec![2, 2, 2, 2, 2, 2, 2, 2],
            ])),
            4  => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            5  => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            6  => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            7  => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            8  => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            9  => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            10 => Bricks::new(ctx, Bricks::make_pattern(vec![])),
            x => panic!("level outside of range!"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Brick
{
    pub rect: Rect,
    pub count: u32,
}
