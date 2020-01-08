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

use std::time::{Instant, Duration};

mod utils;
mod paddle;
mod ball;
mod bricks;
mod pause_ui;
mod forehead;

use paddle::Paddle;
use ball::Ball;
use bricks::Bricks;
use pause_ui::PauseUI;
use forehead::ForeHead;

const LINE_LENGTH: f32 = 30.0;
const ANGLE_CHANGE: f32 = 0.04;

pub struct Game
{
    paddle: Paddle,
    ball: Option<Ball>,
    bricks: Bricks,
    pause_ui: PauseUI,
    forehead: ForeHead,

    phase: Phase,

    game_data: GameData,
    input_data: InputData,
}

impl Game
{
    pub fn new(ctx: &mut Context) -> GameResult<Game>
    {
        let bricks = Bricks::new(ctx, Bricks::make_pattern(vec![
            vec![],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        ]))?;

        let max_score = bricks.len();

        Ok(Game
        {
            paddle: Paddle::new(ctx)?,
            ball: None,
            bricks: bricks,
            pause_ui: PauseUI::new(ctx)?,
            forehead: ForeHead::new(ctx, max_score)?,

            phase: Phase::Shoot(utils::normalize([0.0, -1.0])),

            game_data: GameData
            {
                score: 0,
                timer: 0,

                paddle_speed: 3.5f32,
                pause_transition: false,

                start_inst: None,
                pause_inst: None,
                pause_dur: Duration::new(0, 0),
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

                    if let Some(pause_inst) = self.game_data.pause_inst
                    {
                        self.game_data.pause_dur += Instant::now()
                            .duration_since(pause_inst);
                        self.game_data.pause_inst = None;
                    }

                    if let None = self.game_data.start_inst
                    {
                        self.game_data.start_inst = Some(Instant::now());
                    }

                    self.phase = Phase::Bounce;
                }
            }
            Phase::Bounce =>
            {
                if let Some(start_inst) = self.game_data.start_inst
                {
                    let game_dur = Instant::now()
                        .duration_since(start_inst) - self.game_data.pause_dur;

                    if  game_dur.as_secs() as u32 > self.game_data.timer
                    {
                        self.game_data.timer = game_dur.as_secs() as u32;
                        self.forehead.set_timer(ctx, self.game_data.timer);
                    }
                }

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
                    let r = ball.update(&self.paddle, &mut self.bricks);
                    if r.destroyed_ball
                    {
                        self.game_data.pause_inst = Some(Instant::now());
                        self.ball = None;
                        self.phase = Phase::Shoot([0.0, -1.0])
                    }
                    if r.destroyed_brick
                    {
                        self.game_data.score += 1;
                        self.forehead.set_score(ctx, self.game_data.score);
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
                        self.game_data.pause_inst = Some(Instant::now());
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
                        if let Some(pause_inst) = self.game_data.pause_inst
                        {
                            self.game_data.pause_dur += Instant::now()
                                .duration_since(pause_inst);
                            self.game_data.pause_inst = None;
                        }
                        self.game_data.pause_transition = true;
                    }
                }

                if self.pause_ui.resume_click()
                {
                    self.pause_ui.reset();
                    if let Some(pause_inst) = self.game_data.pause_inst
                    {
                        self.game_data.pause_dur += Instant::now()
                            .duration_since(pause_inst);
                        self.game_data.pause_inst = None;
                    }
                    self.phase = Phase::Bounce;
                }
                if self.pause_ui.restart_click()
                {
                    self.reset();
                }
                if self.pause_ui.main_menu_click()
                {
                    unimplemented!()
                }
            },
        }
        
        Ok(())
    }

    pub fn reset(&mut self)
    {
        self.paddle.reset();
        self.ball = None;
        self.bricks.reset();
        self.pause_ui.reset();

        self.phase = Phase::Shoot(utils::normalize([0.0, -1.0]));

        self.game_data = GameData
        {
            paddle_speed: 3.5f32,
            pause_transition: false,
            score: 0,
            timer: 0,
            start_inst: None,
            pause_inst: None,
            pause_dur: Duration::new(0, 0),
        };
        self.input_data = InputData
        {
            left_down: false,
            right_down: false,
            enter_down: false,
            p_down: false,
        };
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
        self.forehead.draw(ctx)?;

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
    score: u32,
    timer: u32,
    
    paddle_speed: f32,
    pause_transition: bool,

    start_inst: Option<Instant>,
    pause_inst: Option<Instant>,
    pause_dur: Duration,
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
