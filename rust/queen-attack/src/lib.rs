#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(Self(rank, file))
        } else {
            None
        }
    }

    pub fn index(&self) -> i32 {
        self.0 * 8 + self.1
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dist = (self.position.index() - other.position.index()).abs();
        self.position.0 == other.position.0
            || self.position.1 == other.position.1
            || dist % 9 == 0
            || dist % 7 == 0
    }
}
