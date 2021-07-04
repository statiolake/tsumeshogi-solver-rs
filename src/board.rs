use crate::piece::PieceKind;
use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    piece: PieceKind,
    pid: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    cells: [Option<Cell>; 81],
    usable: [Vec<PieceKind>; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    board: Board,
    turn: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [None; 81],
            usable: [vec![], vec![]],
        }
    }

    pub fn is_movable(&self, pid: usize, orig: Position, target: Position) -> bool {
        // 元の駒が存在して自分の駒であることを確認
        let orig_is_mine = self.cells[orig.index()]
            .map(|cell| cell.pid == pid)
            .unwrap_or(false);
        // 行き先が敵の駒または空白であることを確認
        let target_enemy_or_empty = self.cells[target.index()]
            .map(|cell| cell.pid != pid)
            .unwrap_or(true);
        // 動き方がルールに沿っていることを確認
        let move_following_rule = self.cells[orig.index()]
            .map(|cell| cell.piece.is_movable(pid, orig, target))
            .unwrap(); // 存在することは確認済み

        orig_is_mine && target_enemy_or_empty && move_following_rule
    }

    pub fn do_move(&mut self, pid: usize, orig: Position, target: Position) {
        assert!(self.is_movable(pid, orig, target));

        // 敵の駒があれば保存して移動
        self.usable[pid].extend(self.cells[target.index()].map(|c| c.piece));
        self.cells[target.index()] = self.cells[orig.index()];
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
            board: Board::new(),
            turn: 0,
        }
    }

    pub fn do_move(&mut self, pid: usize, from: Position, to: Position) {
        todo!()
    }
}
