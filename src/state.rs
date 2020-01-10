use crate::game::Game;

pub enum State
{
    // the main menu
    Menu,
    // the level select menu
    LevelSelect,
    // the gameplay
    Game(Game),
    // the level clear screen
    LevelClear,
    // the level lose screen
    LevelLose,
}
