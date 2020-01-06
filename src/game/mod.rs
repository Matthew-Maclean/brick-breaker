use ggez::
{
    Context,
    GameResult,
    input::
    {
        keyboard::KeyCode,
    }
};

mod utils;
mod paddle;
mod ball;
mod bricks;

use paddle::Paddle;
use ball::Ball;
use bricks::Bricks;

pub struct Game
{
    paddle: Paddle,
    ball: Ball,
    bricks: Bricks,

    game_data: GameData,
    input_data: InputData,
}

impl Game
{
    pub fn new(ctx: &mut Context) -> GameResult<Game>
    {
        Ok(Game
        {
            paddle: Paddle::new(ctx)?,
            ball: Ball::new(ctx, [250f32, 250f32], [1.0, -1.2])?,
            bricks: Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
            ]))?,
            
            game_data: GameData
            {
                paddle_speed: 5f32,
            },
            input_data: InputData
            {
                left_down: false,
                right_down: false,
            },
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()>
    {
        if self.input_data.left_down
        {
            self.paddle.shift(-self.game_data.paddle_speed);
        }
        if self.input_data.right_down
        {
            self.paddle.shift(self.game_data.paddle_speed);
        }

        self.ball.update(&self.paddle, &mut self.bricks);

        Ok(())
    }

    pub fn key_down(&mut self, key: KeyCode, _repeat: bool)
    {
        match key
        {
            KeyCode::Left => self.input_data.left_down = true,
            KeyCode::Right => self.input_data.right_down = true,
            _ => { }
        }
    }

    pub fn key_up(&mut self, key: KeyCode)
    {
        match key
        {
            KeyCode::Left => self.input_data.left_down = false,
            KeyCode::Right => self.input_data.right_down = false,
            _ => { }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        self.paddle.draw(ctx)?;
        self.ball.draw(ctx)?;
        self.bricks.draw(ctx)
    }
}

struct GameData
{
    paddle_speed: f32,
}

struct InputData
{
    left_down: bool,
    right_down: bool,
}

