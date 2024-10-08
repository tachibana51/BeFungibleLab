// src/interfaces/mod.rs

pub mod command_registry;
pub mod console_io_handler;
pub mod io_handler;

// 再エクスポート（必要に応じて）
pub use command_registry::CommandResolve;
pub use console_io_handler::ConsoleIOHandler;
pub use io_handler::IOHandle;
