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

const STARTING_LIVES: u32 = 3;

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
    pub fn new(ctx: &mut Context, bricks: Vec<Vec<u32>>) -> GameResult<Game>
    {
        let bricks = Bricks::new(ctx, Bricks::make_pattern(bricks))?;

        let max_score = bricks.total();

        Ok(Game
        {
            paddle: Paddle::new(ctx)?,
            ball: None,
            bricks: bricks,
            pause_ui: PauseUI::new(ctx)?,
            forehead: ForeHead::new(ctx, max_score, STARTING_LIVES)?,

            phase: Phase::Shoot(utils::normalize([0.0, -1.0])),

            game_data: GameData
            {
                score: 0,
                max_score: max_score,
                timer: 0,
                
                lives: STARTING_LIVES,
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
            // the game starts in this phase, and returns to it if the ball is destroyed
            Phase::Shoot(ref mut angle) =>
            {
                // rotate the angle of the ball shooter
                if self.input_data.left_down
                {
                    *angle = utils::rotate(*angle, -ANGLE_CHANGE);
                }
                if self.input_data.right_down
                {
                    *angle = utils::rotate(*angle, ANGLE_CHANGE);
                }

                // confine the angle to between [-1.0, -0.15] to [1.0, -0.15]
                // (y = 0 is the top of the window)
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
                
                // shoot the ball
                if self.input_data.enter_down
                {
                    // the ball has the angle of the current shooter angle,
                    // and starts right above the center of the paddle
                    self.ball = Some(Ball::new(
                        ctx,
                        [
                            self.paddle.rect().x + self.paddle.rect().w / 2.0,
                            self.paddle.rect().y + ball::BALL_SIZE
                        ],
                        *angle)?);
                    
                    // if the pause_inst is Some, then we have returned to this phase
                    // after the ball was destroyed, so we need to update the pause
                    // duration to keep the timer accurate
                    if let Some(pause_inst) = self.game_data.pause_inst
                    {
                        self.game_data.pause_dur += Instant::now()
                            .duration_since(pause_inst);
                        // also unset the last pause instance
                        self.game_data.pause_inst = None;
                    }
                    
                    // if the start_inst is none, then this is the first time we've been in this
                    // phase, so this marks the start of the timer
                    // (this should not be true if pause_inst is Some)
                    if let None = self.game_data.start_inst
                    {
                        self.game_data.start_inst = Some(Instant::now());
                    }

                    // move to the next phase
                    self.phase = Phase::Bounce;
                }
            }
            // this is the main gameplay phase
            Phase::Bounce =>
            {
                // update the timer
                if let Some(start_inst) = self.game_data.start_inst
                {
                    // the current game duration, minus the time spent paused
                    let game_dur = Instant::now()
                        .duration_since(start_inst) - self.game_data.pause_dur;
                    
                    // the timer is in seconds only, so only update it if at least one
                    // more second has passed
                    if  game_dur.as_secs() as u32 > self.game_data.timer
                    {
                        self.game_data.timer = game_dur.as_secs() as u32;
                        self.forehead.set_timer(ctx, self.game_data.timer);
                    }
                }
                
                // move the paddle, the paddle will handle confining itself to the board
                if self.input_data.left_down
                {
                    self.paddle.shift(-self.game_data.paddle_speed);
                }
                if self.input_data.right_down
                {
                    self.paddle.shift(self.game_data.paddle_speed);
                }
                
                // this should always be true in the bounce phase
                // (maybe .expect() it?)
                if let Some(ref mut ball) = &mut self.ball
                {
                    // the ball update handles all bouncing, including going off the board
                    // and being destroyed, and bouncing off of / breaking bricks
                    let r = ball.update(&self.paddle, &mut self.bricks);
                    if r.destroyed_ball
                    {
                        // lose a life and update the forehead
                        self.game_data.lives -= 1;
                        self.forehead.set_lives(self.game_data.lives);
                        if self.game_data.lives == 0
                        {
                            unimplemented!()
                        }
                        // pause the timer
                        self.game_data.pause_inst = Some(Instant::now());
                        // get rid of the ball so it isn't drawn
                        self.ball = None;
                        // go back to the shooting phase
                        self.phase = Phase::Shoot([0.0, -1.0])
                    }
                    if r.collided_brick
                    {
                        // up the score
                        self.game_data.score += 1;
                        // update the score in the forehead
                        self.forehead.set_score(ctx, self.game_data.score);
                        // >= for safety
                        if self.game_data.score >= self.game_data.max_score
                        {
                            unimplemented!()
                        }
                    }
                }
                
                // if pause_transition is still set from the pause phase, and the
                // 'p' key has been released, unset pause_transition
                if self.game_data.pause_transition
                {
                    if !self.input_data.p_down
                    {
                        self.game_data.pause_transition = false;
                    }
                }
                
                // if pause_transition is unset, and the 'p' is pressed, go the the pause phase
                if !self.game_data.pause_transition
                {
                    if self.input_data.p_down
                    {
                        // change the phase
                        self.phase = Phase::Pause;
                        // pause the timer
                        self.game_data.pause_inst = Some(Instant::now());
                        // since this pause comes from the key press, set pause_transition
                        // so that the game won't immediately unpause
                        self.game_data.pause_transition = true;
                    }
                }
            },
            Phase::Pause =>
            {
                // if pause_transiton is set, and the 'p' key has been released,
                // unset pause_transition
                if self.game_data.pause_transition
                {
                    if !self.input_data.p_down
                    {
                        self.game_data.pause_transition = false;
                    }
                }
                
                // if the pause_transiton is unset, and the 'p' key is pressed, unpause
                if !self.game_data.pause_transition
                {
                    if self.input_data.p_down
                    {
                        // change the phase
                        self.phase = Phase::Bounce;
                        // if pause_inst is Some, update the pause duration
                        // (this should always be set, maybe .expect() it?)
                        if let Some(pause_inst) = self.game_data.pause_inst
                        {
                            self.game_data.pause_dur += Instant::now()
                                .duration_since(pause_inst);
                            self.game_data.pause_inst = None;
                        }
                        // 'p' was pressed, so set pause_transition
                        self.game_data.pause_transition = true;
                    }
                }
                
                // the other way of unpausing, do the same stuff as before but don't
                // set pause_transition, since the 'p' key wasn't pressed
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
                    // restarts the whole thing
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
            max_score: self.bricks.total(),
            timer: 0,
            lives: STARTING_LIVES,
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
        // the paddle, bricks, ball, and forehead are always drawn,
        // we draw them before so that if something (like the pause menu)
        // wants to cover them it can
        self.paddle.draw(ctx)?;
        if let Some(ref ball) = &self.ball
        {
            ball.draw(ctx)?;
        }
        self.bricks.draw(ctx)?;
        self.forehead.draw(ctx)?;

        match &self.phase
        {
            // in she shoot phase, draw the ball shooter 
            Phase::Shoot(ref angle) =>
            {
                use ggez::graphics;

                let start =
                [
                    self.paddle.rect().x + self.paddle.rect().w / 2.0,
                    self.paddle.rect().y
                ];
                
                // it's a line
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
            // the bounce phase has nothing special to draw, as everything is already drawn
            Phase::Bounce => { },
            // in this phase we draw the pause menu
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
    max_score: u32,
    timer: u32,

    lives: u32,
    paddle_speed: f32,
    pause_transition: bool,
    
    // the time at the start of the game
    start_inst: Option<Instant>,
    // the last time the timer was paused
    pause_inst: Option<Instant>,
    // the total pause time of the game
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
