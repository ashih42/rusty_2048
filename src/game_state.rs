use savefile::savefile_derive::Savefile;

#[derive(Clone, Debug, PartialEq, Savefile)]
pub enum GameState {
    InPlay,
    Won,
    Lost,
}
