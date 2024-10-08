// src/entities/mod.rs

pub mod code_grid;
pub mod direction;
pub mod ip_state;

// 再エクスポート（必要に応じて）
pub use code_grid::CodeGrid;
pub use direction::Direction;
pub use ip_state::IPState;
