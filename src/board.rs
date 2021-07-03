use crate::piece::PieceKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize);

impl Position {
    pub fn new(index: usize) -> Position {
        assert!(index < 81);
        Position(index)
    }

    pub fn from_ij(i: usize, j: usize) -> Option<Position> {
        if i < 9 && j < 9 {
            Some(Position(i * 9 + j))
        } else {
            None
        }
    }

    pub fn to_i(self) -> usize {
        self.0 / 9
    }

    pub fn to_j(self) -> usize {
        self.0 % 9
    }

    pub fn to_ij(self) -> (usize, usize) {
        (self.to_i(), self.to_j())
    }

    pub fn add(self, di: isize, dj: isize) -> Option<Position> {
        let (i, j) = self.to_ij();
        Self::from_ij((i as isize + di) as usize, (j as isize + dj) as usize)
    }

    pub fn on_kaku(self, other: Self) -> bool {
        let (ai, aj) = self.to_ij();
        let (bi, bj) = self.to_ij();

        (ai as isize - bi as isize).abs() == (aj as isize - bj as isize).abs()
    }

    pub fn on_hisha(self, other: Self) -> bool {
        let (ai, aj) = self.to_ij();
        let (bi, bj) = self.to_ij();

        ai == bi || aj == bj
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    field: [Option<PieceKind>; 81],
    puttable: Vec<PieceKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    boards: [Board; 2],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            field: [None; 81],
            puttable: vec![],
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            boards: [Board::new(), Board::new()],
        }
    }

    pub fn composite_field(&self) -> [Option<(PieceKind, usize)>; 81] {
        let mut res = [None; 81];
        for (idx, (a, b)) in
            Iterator::zip(self.boards[0].field.iter(), self.boards[1].field.iter()).enumerate()
        {
            res[idx] = a.map(|k| (k, 0));
            res[opponent_pos(idx)] = b.map(|k| (k, 1));
        }

        res
    }

    pub fn do_move(&mut self, pid: usize, from: usize, to: usize) {
        todo!()
    }
}
