#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize);

impl Position {
    pub fn new(index: usize) -> Position {
        assert!(index < 81);
        Position(index)
    }

    pub fn index(self) -> usize {
        self.0
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
