use savefile::savefile_derive::Savefile;

#[derive(Clone, Debug, PartialEq, Eq, Savefile)]
pub enum GameState {
    InPlay,
    Won,
    Lost,
}
