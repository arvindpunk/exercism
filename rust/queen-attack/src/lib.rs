#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition{rank, file})
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen{pos: position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.rank == other.pos.rank ||
        self.pos.file == other.pos.file ||
        i32::abs(self.pos.rank - other.pos.rank) == i32::abs(self.pos.file - other.pos.file)
    }
}