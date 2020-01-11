use crate::
{
    menu::Menu,
    level_select::LevelSelect,
    game::Game,
    level_clear::LevelClear,
};

pub enum State
{
    // the main menu
    Menu(Menu),
    // the level select menu
    LevelSelect(LevelSelect),
    // the gameplay
    Game(Game),
    // the level clear screen
    LevelClear(LevelClear),
    // the level lose screen
    LevelLose,
}
