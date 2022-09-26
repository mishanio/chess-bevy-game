#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub enum ChessColor {
    #[default]
    WHITE,
    BLACK,
}

impl ChessColor {
    pub fn opposite(&self) -> Self {
        match self {
            ChessColor::WHITE => ChessColor::BLACK,
            ChessColor::BLACK => ChessColor::WHITE,
        }
    }
}
