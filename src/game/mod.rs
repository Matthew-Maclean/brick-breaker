use ggez::
{
    Context,
    GameResult,
    input::
    {
        keyboard::KeyCode,
        mouse::MouseButton,
    },
};

mod utils;
mod paddle;
mod ball;
mod bricks;
mod pause_ui;

use paddle::Paddle;
use ball::Ball;
use bricks::Bricks;
use pause_ui::PauseUI;

const LINE_LENGTH: f32 = 30.0;
const ANGLE_CHANGE: f32 = 0.04;

pub struct Game
{
    paddle: Paddle,
    ball: Option<Ball>,
    bricks: Bricks,
    pause_ui: PauseUI,

    phase: Phase,

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
            ball: None,
            bricks: Bricks::new(ctx, Bricks::make_pattern(vec![
                vec![],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
            ]))?,
            pause_ui: PauseUI::new(ctx)?,

            phase: Phase::Shoot(utils::normalize([0.0, -1.0])),

            game_data: GameData
            {
                paddle_speed: 3.5f32,
                pause_transition: false,
            },
            input_data: InputData
            {
                left_down: false,
                right_down: false,
                enter_down: false,
                p_down: false,
            },
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        match &mut self.phase
        {
            Phase::Shoot(ref mut angle) =>
            {
                if self.input_data.left_down
                {
                    *angle = utils::rotate(*angle, -ANGLE_CHANGE);
                }
                if self.input_data.right_down
                {
                    *angle = utils::rotate(*angle, ANGLE_CHANGE);
                }

                if utils::angle_between(*angle, [1.0, 0.0]) <
                    utils::angle_between([1.0, -0.15], [1.0, 0.0])
                {
                    *angle = [1.0, -0.15];
                }
                if utils::angle_between(*angle, [-1.0, 0.0]) <
                    utils::angle_between([-1.0, -0.15], [-1.0, 0.0])
                {
                    *angle = [-1.0, -0.15]
                }

                if self.input_data.enter_down
                {
                    self.ball = Some(Ball::new(
                        ctx,
                        [
                            self.paddle.rect().x + self.paddle.rect().w / 2.0,
                            self.paddle.rect().y + ball::BALL_SIZE
                        ],
                        *angle)?);
                    self.phase = Phase::Bounce;
                }
            }
            Phase::Bounce =>
            {
                if self.input_data.left_down
                {
                    self.paddle.shift(-self.game_data.paddle_speed);
                }
                if self.input_data.right_down
                {
                    self.paddle.shift(self.game_data.paddle_speed);
                }

                if let Some(ref mut ball) = &mut self.ball
                {
                    if !ball.update(&self.paddle, &mut self.bricks)
                    {
                        self.ball = None;
                        self.phase = Phase::Shoot([0.0, -1.0])
                    }
                }

                if self.game_data.pause_transition
                {
                    if !self.input_data.p_down
                    {
                        self.game_data.pause_transition = false;
                    }
                }
                
                if !self.game_data.pause_transition
                {
                    if self.input_data.p_down
                    {
                        self.phase = Phase::Pause;
                        self.game_data.pause_transition = true;
                    }
                }
            },
            Phase::Pause =>
            {
                if self.game_data.pause_transition
                {
                    if !self.input_data.p_down
                    {
                        self.game_data.pause_transition = false;
                    }
                }

                if !self.game_data.pause_transition
                {
                    if self.input_data.p_down
                    {
                        self.phase = Phase::Bounce;
                        self.game_data.pause_transition = true;
                    }
                }

                if self.pause_ui.resume_click()
                {
                    self.pause_ui.reset();

                    self.phase = Phase::Bounce;
                }
                if self.pause_ui.restart_click()
                {
                    // todo
                }
                if self.pause_ui.main_menu_click()
                {
                    unimplemented!()
                }
            },
        }
        
        Ok(())
    }

    pub fn key_down(&mut self, key: KeyCode, _repeat: bool)
    {
        match key
        {
            KeyCode::Left => self.input_data.left_down = true,
            KeyCode::Right => self.input_data.right_down = true,
            KeyCode::Return => self.input_data.enter_down = true,
            KeyCode::P => self.input_data.p_down = true,
            _ => { }
        }
    }

    pub fn key_up(&mut self, key: KeyCode)
    {
        match key
        {
            KeyCode::Left => self.input_data.left_down = false,
            KeyCode::Right => self.input_data.right_down = false,
            KeyCode::Return => self.input_data.enter_down = false,
            KeyCode::P => self.input_data.p_down = false,
            _ => { }
        }
    }

    pub fn mouse_move(&mut self, x: f32, y: f32)
    {
        match self.phase
        {
            Phase::Pause =>
            {
                self.pause_ui.mouse_move(x, y);
            },
            _ => { }
        }
    }

    pub fn mouse_down(&mut self, button: MouseButton, x: f32, y: f32)
    {
        match self.phase
        {
            Phase::Pause =>
            {
                self.pause_ui.mouse_down(button, x, y);
            },
            _ => { }
        }
    }

    pub fn mouse_up(&mut self, button: MouseButton)
    {
        match self.phase
        {
            Phase::Pause =>
            {
                self.pause_ui.mouse_up(button);
            },
            _ => { }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        self.paddle.draw(ctx)?;
        if let Some(ref ball) = &self.ball
        {
            ball.draw(ctx)?;
        }
        self.bricks.draw(ctx)?;

        match &self.phase
        {
            Phase::Shoot(ref angle) =>
            {
                use ggez::graphics;

                let start =
                [
                    self.paddle.rect().x + self.paddle.rect().w / 2.0,
                    self.paddle.rect().y
                ];

                let line = graphics::Mesh::new_line(
                    ctx,
                    &[
                        start,
                        [start[0] + angle[0] * LINE_LENGTH, start[1] + angle[1] * LINE_LENGTH]
                    ],
                    1.0,
                    graphics::BLACK)?;

                graphics::draw(ctx, &line, graphics::DrawParam::new())?;
            },
            Phase::Bounce => { },
            Phase::Pause =>
            {
                self.pause_ui.draw(ctx)?;
            },
        }

        Ok(())
    }
}

struct GameData
{
    paddle_speed: f32,
    pause_transition: bool,
}

struct InputData
{
    left_down: bool,
    right_down: bool,
    enter_down: bool,
    p_down: bool,
}

enum Phase
{
    Shoot([f32; 2]),
    Bounce,
    Pause,
}
