#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Startup,
    Splashscreen,
    MainMenu,
    Dialogue,
    Freemove,
}
