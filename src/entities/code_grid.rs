use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::errors::InterpreterError;

#[derive(Clone)]
pub struct CodeGrid {
    pub grid: Arc<Mutex<Vec<Vec<char>>>>,
    pub code_width: usize,
    pub code_height: usize,
}

impl CodeGrid {
    pub fn new(grid: Vec<Vec<char>>, code_width: usize, code_height: usize) -> Self {
        Self {
            grid: Arc::new(Mutex::new(grid)),
            code_width,
            code_height,
        }
    }
    pub fn load(file_path: &str) -> Result<Self, InterpreterError> {
        // ファイルパスが存在するか確認
        if !Path::new(file_path).exists() {
            return Err(InterpreterError::FileNotFound(format!(
                "File not found: {}",
                file_path
            )));
        }

        // ファイルの内容を読み込む
        let content = fs::read_to_string(file_path).map_err(|e| {
            InterpreterError::FileReadError(format!("Failed to read file {}: {}", file_path, e))
        })?;

        // 各行をベクターに分割
        let lines: Vec<&str> = content.lines().collect();

        // 最長行の長さを取得
        let max_width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        // グリッドの高さは行数、幅は最長行に合わせる
        let code_height = lines.len();
        let code_width = max_width;

        // グリッドを初期化（パディングはスペースで埋める）
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(code_height);

        for line in lines {
            let mut row: Vec<char> = line.chars().collect();
            if row.len() < max_width {
                row.extend(std::iter::repeat(' ').take(max_width - row.len()));
            }
            grid.push(row);
        }

        // スタブ用に最低1x1のグリッドを確保
        if grid.is_empty() {
            grid.push(vec![' '; 1]);
        }

        Ok(Self::new(grid, code_width, code_height))
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::CodeGrid;
    use crate::errors::InterpreterError;

    #[test]
    fn test_load_program_success() {
        let file_path = "test_program.bef";

        // テスト用のファイルを作成
        std::fs::write(file_path, ">1+2@ ").expect("Failed to create test_program.bef");

        // Program::load を呼び出す
        let program = CodeGrid::load(file_path);

        // テスト用のファイルを削除
        std::fs::remove_file(file_path).expect("Failed to remove test_program.bef");

        // 結果を検証
        assert!(program.is_ok(), "Program::load should succeed");
        let program = program.unwrap();
        assert_eq!(program.code_height, 1, "Code height should be 1");
        assert_eq!(program.code_width, 6, "Code width should be 6");

        let grid = program.grid.lock().unwrap();
        assert_eq!(grid.len(), 1, "Grid should have 1 row");
        assert_eq!(
            grid[0],
            vec!['>', '1', '+', '2', '@', ' '],
            "Grid row should match"
        );
    }

    #[test]
    fn test_load_program_empty_file() {
        let file_path = "empty.bef";

        // テスト用の空ファイルを作成
        std::fs::write(file_path, "").expect("Failed to create empty.bef");

        // Program::load を呼び出す
        let program = CodeGrid::load(file_path);

        // テスト用のファイルを削除
        std::fs::remove_file(file_path).expect("Failed to remove empty.bef");

        // 結果を検証
        assert!(
            program.is_ok(),
            "Program::load should succeed even for empty file"
        );
        let program = program.unwrap();
        assert_eq!(program.code_height, 0, "Code height should be at least 0");
        assert_eq!(program.code_width, 0, "Code width should be at least 0");

        let grid = program.grid.lock().unwrap();
        assert_eq!(grid.len(), 1, "Grid should have 1 row");
        assert_eq!(grid[0], vec![' '], "Grid row should contain a single space");
    }

    #[test]
    fn test_load_program_nonexistent_file() {
        let file_path = "nonexistent.bef";

        // Program::load を呼び出す
        let program = CodeGrid::load(file_path);

        // 結果を検証
        assert!(
            matches!(program, Err(InterpreterError::FileNotFound(_))),
            "Program::load should return FileNotFound error for nonexistent file"
        );
    }

    #[test]
    fn test_load_program_with_different_line_lengths() {
        let file_path = "test_multiline.bef";

        // テスト用のマルチラインファイルを作成
        std::fs::write(file_path, ">1+\n2@ ").expect("Failed to create test_multiline.bef");

        // Program::load を呼び出す
        let program = CodeGrid::load(file_path);

        // テスト用のファイルを削除
        std::fs::remove_file(file_path).expect("Failed to remove test_multiline.bef");

        // 結果を検証
        assert!(
            program.is_ok(),
            "Program::load should succeed for multiline file"
        );
        let program = program.unwrap();
        assert_eq!(program.code_height, 2, "Code height should be 2");
        assert_eq!(
            program.code_width, 3,
            "Code width should be 3 (max line length)"
        );

        let grid = program.grid.lock().unwrap();
        assert_eq!(grid.len(), 2, "Grid should have 2 rows");
        assert_eq!(grid[0], vec!['>', '1', '+'], "First row should match");
        assert_eq!(
            grid[1],
            vec!['2', '@', ' '],
            "Second row should be padded with space"
        );
    }
}
