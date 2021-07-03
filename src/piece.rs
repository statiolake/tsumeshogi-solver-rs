use crate::board::Position;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Fu,
    Ky,
    Ke,
    Gi,
    Ki,
    Ka,
    Hi,
    Ou,
    PFu,
    PKy,
    PKe,
    PGi,
    PKa,
    PHi,
}

const DIJ_FU: &[(isize, isize)] = &[(-1, 0)];
const DIJ_KY: &[(isize, isize)] = &[
    (-1, 0),
    (-2, 0),
    (-3, 0),
    (-4, 0),
    (-5, 0),
    (-6, 0),
    (-7, 0),
    (-8, 0),
];
const DIJ_KE: &[(isize, isize)] = &[(-2, -1), (-2, 1)];
const DIJ_GI: &[(isize, isize)] = &[(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 1)];
const DIJ_KI: &[(isize, isize)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0)];
const DIJ_KA: &[(isize, isize)] = &[
    (-8, -8),
    (-7, -7),
    (-6, -6),
    (-5, -5),
    (-4, -4),
    (-3, -3),
    (-2, -2),
    (-1, -1),
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (6, 6),
    (7, 7),
    (8, 8),
    (-8, 8),
    (-7, 7),
    (-6, 6),
    (-5, 5),
    (-4, 4),
    (-3, 3),
    (-2, 2),
    (-1, 1),
    (1, -1),
    (2, -2),
    (3, -3),
    (4, -4),
    (5, -5),
    (6, -6),
    (7, -7),
    (8, -8),
];
const DIJ_HI: &[(isize, isize)] = &[
    (-8, 0),
    (-7, 0),
    (-6, 0),
    (-5, 0),
    (-4, 0),
    (-3, 0),
    (-2, 0),
    (-1, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (5, 0),
    (6, 0),
    (7, 0),
    (8, 0),
    (0, -8),
    (0, -7),
    (0, -6),
    (0, -5),
    (0, -4),
    (0, -3),
    (0, -2),
    (0, -1),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (0, 7),
    (0, 8),
];
const DIJ_OU: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const DIJ_PKA: &[(isize, isize)] = &[
    // 角の動き
    (-8, -8),
    (-7, -7),
    (-6, -6),
    (-5, -5),
    (-4, -4),
    (-3, -3),
    (-2, -2),
    (-1, -1),
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (6, 6),
    (7, 7),
    (8, 8),
    (-8, 8),
    (-7, 7),
    (-6, 6),
    (-5, 5),
    (-4, 4),
    (-3, 3),
    (-2, 2),
    (-1, 1),
    (1, -1),
    (2, -2),
    (3, -3),
    (4, -4),
    (5, -5),
    (6, -6),
    (7, -7),
    (8, -8),
    // 追加の動き
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 0),
];
const DIJ_PHI: &[(isize, isize)] = &[
    // 飛車の動き
    (-8, 0),
    (-7, 0),
    (-6, 0),
    (-5, 0),
    (-4, 0),
    (-3, 0),
    (-2, 0),
    (-1, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (5, 0),
    (6, 0),
    (7, 0),
    (8, 0),
    (0, -8),
    (0, -7),
    (0, -6),
    (0, -5),
    (0, -4),
    (0, -3),
    (0, -2),
    (0, -1),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (0, 7),
    (0, 8),
    // 追加の動き
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

impl PieceKind {
    pub fn is_movable(self, pid: usize, p: Position, t: Position) -> bool {
        // 同じ場所なら無理
        if p == t {
            return false;
        }

        let mul = (-1isize).pow(pid as u32);
        let (pi, pj) = {
            let (i, j) = p.to_ij();
            (i as isize, j as isize)
        };
        let (ti, tj) = {
            let (i, j) = p.to_ij();
            (i as isize, j as isize)
        };

        // 香車・角・飛車などのちょっとした高速化
        use PieceKind::*;
        match self {
            // 横移動無しで今の位置よりも前側に居たら OK
            Ky => (tj == pj && ti * mul < pi * mul),
            // 縦と横の差分の絶対値が一致していれば斜め移動
            Ka => (ti - pi).abs() == (tj - pj).abs(),
            // 縦または横が一致していれば OK
            Hi => ti == pi || tj == pj,
            // マンハッタン距離が 1 なら OK
            Ou => max((ti - pi).abs(), (tj - pj).abs()) == 1,
            // 角または王
            PKa => (ti - pi).abs() == (tj - pj).abs() || max((ti - pi).abs(), (tj - pj).abs()) == 1,
            // 飛車または王
            PHi => (ti == pi || tj == pj) || max((ti - pi).abs(), (tj - pj).abs()) == 1,
            // あとは不規則なので諦める
            _ => self.movable_cells(pid, p).contains(&t),
        }
    }

    pub fn movable_cells(self, pid: usize, p: Position) -> Vec<Position> {
        use PieceKind::*;

        let mul = (-1isize).pow(pid as u32);
        let dij = match self {
            Fu => DIJ_FU,
            Ky => DIJ_KY,
            Ke => DIJ_KE,
            Gi => DIJ_GI,
            Ki | PFu | PKy | PKe | PGi => DIJ_KI,
            Ka => DIJ_KA,
            Hi => DIJ_HI,
            Ou => DIJ_OU,
            PKa => DIJ_PKA,
            PHi => DIJ_PHI,
        };

        dij.iter()
            .flat_map(|&(di, dj)| p.add(di * mul, dj))
            .collect()
    }
}
