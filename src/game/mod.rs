use ggez::
{
    Context,
    GameResult,
    input::
    {
        keyboard::KeyCode,
    }
};

mod paddle;

use paddle::Paddle;

pub struct Game
{
    paddle: Paddle,

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
        self.paddle.draw(ctx)
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

