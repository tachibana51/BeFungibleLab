// src/entities/direction.rs

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    /// 方向に応じた dx を返します。
    pub fn dx(&self) -> isize {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            Direction::Up => 0,
            Direction::Down => 0,
        }
    }

    /// 方向に応じた dy を返します。
    pub fn dy(&self) -> isize {
        match self {
            Direction::Right => 0,
            Direction::Left => 0,
            Direction::Up => -1,
            Direction::Down => 1,
        }
    }
    pub fn from_dx_dy(dx: isize, dy: isize) -> Option<Self> {
        match (dx, dy) {
            (1, 0) => Some(Direction::Right),
            (-1, 0) => Some(Direction::Left),
            (0, 1) => Some(Direction::Down),
            (0, -1) => Some(Direction::Up),
            _ => None, // 未定義の方向の場合
        }
    }
}
