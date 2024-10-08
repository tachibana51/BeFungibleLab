// src/entities/ip_state.rs

use crate::entities::Direction;

pub struct IPState {
    pub ip_x: usize,
    pub ip_y: usize,
    pub direction: Direction, // ここを Direction 型に変更
    pub terminated: bool,
    pub stk: Vec<usize>,
    pub string_mode_active: bool,
}

impl IPState {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            ip_x: x,
            ip_y: y,
            direction,
            terminated: false,
            stk: Vec::new(),
            string_mode_active: false,
        }
    }
}
