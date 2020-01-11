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

use crate::{BOARD_WIDTH, FOREHEAD, MAX_LEVEL};

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
            4  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![2, 2, 3, 3, 4, 4, 5, 4, 4, 3, 3, 2, 2],
                vec![3, 3, 2, 2, 1, 1, 0, 1, 1, 2, 2, 3, 3],
                vec![1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1],
            ])),
            5  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
                vec![0, 0, 5, 5, 6, 6, 6, 6, 6, 5, 5, 0, 0],
                vec![4, 5, 4, 5, 4, 5, 6, 5, 4, 5, 4, 5, 4],
            ])),
            6  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![1 , 3 , 6 , 9 , 12, 15, 12, 9 , 6 , 3 , 1 ],
                vec![3 , 6 , 9 , 12, 15, 18, 15, 12, 9 , 6 , 3 ],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
            ])),
            7  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![11, 12, 13, 14, 15, 16, 15, 14, 13, 12, 11],
                vec![11, 13, 15, 17, 19, 0 , 19, 17, 15, 13, 11],
                vec![11, 12, 13, 14, 15, 16, 15, 14, 13, 12, 11],
            ])),
            8  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![14, 16, 14, 0 , 0 , 15, 0 , 0 , 14, 16, 14],
                vec![14, 16, 14, 0 , 0 , 18, 0 , 0 , 14, 16, 14],
                vec![14, 16, 14, 0 , 0 , 18, 0 , 0 , 14, 16, 14],
                vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15],
            ])),
            9  => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![],
                vec![15, 16, 17, 18, 19, 19, 19, 18, 17, 16, 15],
                vec![17, 18, 19, 18, 17, 19, 17, 18, 19, 18, 17],
                vec![19, 18, 17, 16, 15, 19, 15, 16, 17, 18, 19],
                vec![12, 0 , 12, 0 , 12, 0 , 12, 0 , 12, 0 , 12],
            ])),
            10 => Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![20, 0 , 20, 0 , 20, 0 , 20, 0 , 20, 0 , 20],
                vec![19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19],
                vec![20, 19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 19, 20],
                vec![19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19],
                vec![20, 0 , 20, 0 , 20, 0 , 20, 0 , 20, 0 , 20],
                vec![0 , 16, 0 , 16, 0 , 19, 0 , 16, 0 , 16, 0 ],
            ])),
            x => panic!("level outside of range! (was {}, max {})", x, MAX_LEVEL),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Brick
{
    pub rect: Rect,
    pub count: u32,
}
