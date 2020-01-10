use crate::game::Game;
use crate::menu::Menu;

pub enum State
{
    // the main menu
    Menu(Menu),
    // the level select menu
    LevelSelect,
    // the gameplay
    Game(Game),
    // the level clear screen
    LevelClear,
    // the level lose screen
    LevelLose,
}
