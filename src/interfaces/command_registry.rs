// src/interfaces/command_registry.rs

use crate::engine::commands::command::Command;
use crate::engine::commands::digit_command::DigitCommand;
use crate::engine::commands::down_command::DownCommand;
use crate::engine::commands::get_command::GetCommand;
use crate::engine::commands::greater_than_command::GraterThanCommand;
use crate::engine::commands::horizontal_if_command::HorizontalIfCommand;
use crate::engine::commands::left_command::LeftCommand;
use crate::engine::commands::read_number_command::ReadNumberCommand;
use crate::engine::commands::right_command::RightCommand;
use crate::engine::commands::{
    add_command::AddCommand, divide_command::DivideCommand, drop_command::DropCommand,
    duplicate_top_command::DuplicateTopCommand, logical_not_command::LogicalNotCommand,
    modulo_command::ModuloCommand, multiply_command::MultiplyCommand,
    print_char_command::PrintCharCommand, print_number_command::PrintNumberCommand,
    string_mode_command::StringModeCommand, subtract_command::SubtractCommand,
    swap_command::SwapCommand, terminate_command::TerminateCommand, thread_command::ThreadCommand,
    up_command::UpCommand, vertical_if_command::VerticalIfCommand,
};
use crate::engine::commands::{
    put_command::PutCommand, read_character_command::ReadCharacterCommand,
};
use std::collections::HashMap;
use std::sync::Arc;

pub trait CommandResolve {
    fn get_command(&self, cmd: char) -> Option<Arc<dyn Command + Send + Sync>>;
}

pub struct CommandRegistry {
    commands: HashMap<char, Arc<dyn Command + Send + Sync>>,
}

impl CommandResolve for CommandRegistry {
    fn get_command(&self, cmd: char) -> Option<Arc<dyn Command + Send + Sync>> {
        self.commands.get(&cmd).cloned()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut commands: HashMap<char, Arc<dyn Command + Send + Sync>> = HashMap::new();
        commands.insert('+', Arc::new(AddCommand));
        commands.insert('-', Arc::new(SubtractCommand));
        commands.insert('*', Arc::new(MultiplyCommand));
        commands.insert('/', Arc::new(DivideCommand));
        commands.insert('%', Arc::new(ModuloCommand));
        commands.insert(':', Arc::new(DuplicateTopCommand));
        commands.insert('\\', Arc::new(SwapCommand));
        commands.insert('$', Arc::new(DropCommand));
        commands.insert('.', Arc::new(PrintNumberCommand));
        commands.insert(',', Arc::new(PrintCharCommand));
        commands.insert('@', Arc::new(TerminateCommand));
        commands.insert('t', Arc::new(ThreadCommand));
        commands.insert('!', Arc::new(LogicalNotCommand));
        commands.insert('_', Arc::new(HorizontalIfCommand));
        commands.insert('|', Arc::new(VerticalIfCommand));
        commands.insert('>', Arc::new(RightCommand));
        commands.insert('<', Arc::new(LeftCommand));
        commands.insert('^', Arc::new(UpCommand));
        commands.insert('v', Arc::new(DownCommand));
        commands.insert('p', Arc::new(PutCommand));
        commands.insert('g', Arc::new(GetCommand));
        commands.insert('"', Arc::new(StringModeCommand));
        commands.insert('&', Arc::new(ReadNumberCommand));
        commands.insert('~', Arc::new(ReadCharacterCommand));
        commands.insert('`', Arc::new(GraterThanCommand));
        // 数字コマンドを登録
        for digit in 0..=9 {
            commands.insert(
                char::from_digit(digit, 10).unwrap(),
                Arc::new(DigitCommand::new(digit as usize)),
            );
        }
        Self { commands }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::commands::command::CommandGrid;
    use crate::entities::ip_state::IPState;
    use crate::entities::Direction;
    use crate::errors::InterpreterError;
    use crate::interfaces::IOHandle;
    use std::sync::{Arc, Mutex};

    struct MockCommandGrid {
        grid: Mutex<Vec<Vec<char>>>,
    }

    impl MockCommandGrid {
        fn new() -> Self {
            Self {
                grid: Mutex::new(vec![vec![' '; 10]; 10]),
            }
        }

        fn from_grid(grid: Vec<Vec<char>>) -> Self {
            Self {
                grid: Mutex::new(grid),
            }
        }
    }

    impl CommandGrid for MockCommandGrid {
        fn pop(&self, ip: Arc<Mutex<IPState>>) -> Result<usize, InterpreterError> {
            let mut ip_locked = ip.lock().unwrap();
            Ok(ip_locked.stk.pop().unwrap_or(0))
        }

        fn move_ip(&self, _ip: Arc<Mutex<IPState>>) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn add_ip(
            &self,
            _new_ip: Arc<Mutex<IPState>>,
            _io_handler: Arc<dyn IOHandle + Send + Sync>,
        ) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn run_ip(
            self: Arc<Self>,
            _ip: Arc<Mutex<IPState>>,
            _io_handler: Arc<dyn IOHandle + Send + Sync>,
        ) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn set_value(&self, x: usize, y: usize, value: char) -> Result<(), InterpreterError> {
            let mut grid = self.grid.lock().unwrap();
            grid[y][x] = value;
            Ok(())
        }

        fn get_value(&self, x: usize, y: usize) -> Result<char, InterpreterError> {
            let grid = self.grid.lock().unwrap();
            Ok(grid[y][x])
        }
    }

    struct MockIOHandler;

    impl IOHandle for MockIOHandler {
        fn write_output(&self, _output: &str) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn write_error(&self, _error: &str) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn read_number(&self) -> Result<usize, InterpreterError> {
            Ok(0)
        }

        fn read_char(&self) -> Result<char, InterpreterError> {
            Ok(' ')
        }

        fn display_stack(&self, _stack: &[usize]) -> Result<(), InterpreterError> {
            todo!()
        }

        fn display_grid(
            &self,
            _grid: &[Vec<char>],
            _ip_x: usize,
            _ip_y: usize,
        ) -> Result<(), InterpreterError> {
            todo!()
        }
    }

    struct MockIOHandlerWithNumber {
        number: usize,
    }

    impl MockIOHandlerWithNumber {
        fn new(number: usize) -> Self {
            MockIOHandlerWithNumber { number }
        }
    }

    impl IOHandle for MockIOHandlerWithNumber {
        fn write_output(&self, _output: &str) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn write_error(&self, _error: &str) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn read_number(&self) -> Result<usize, InterpreterError> {
            Ok(self.number)
        }

        fn read_char(&self) -> Result<char, InterpreterError> {
            Err(InterpreterError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "read_char not implemented",
            )))
        }

        fn display_stack(&self, _stack: &[usize]) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn display_grid(
            &self,
            _grid: &[Vec<char>],
            _ip_x: usize,
            _ip_y: usize,
        ) -> Result<(), InterpreterError> {
            Ok(())
        }
    }

    struct MockIOHandlerWithChar {
        character: char,
    }

    impl MockIOHandlerWithChar {
        fn new(character: char) -> Self {
            MockIOHandlerWithChar { character }
        }
    }

    impl IOHandle for MockIOHandlerWithChar {
        fn write_output(&self, _output: &str) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn write_error(&self, _error: &str) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn read_number(&self) -> Result<usize, InterpreterError> {
            Err(InterpreterError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "read_number not implemented",
            )))
        }

        fn read_char(&self) -> Result<char, InterpreterError> {
            Ok(self.character)
        }

        fn display_stack(&self, _stack: &[usize]) -> Result<(), InterpreterError> {
            Ok(())
        }

        fn display_grid(
            &self,
            _grid: &[Vec<char>],
            _ip_x: usize,
            _ip_y: usize,
        ) -> Result<(), InterpreterError> {
            Ok(())
        }
    }
    #[test]
    fn test_command_registry_add_command() {
        let registry = CommandRegistry::new();
        let add_command = registry.get_command('+');

        assert!(
            add_command.is_some(),
            "AddCommand should be registered for '+'"
        );
    }

    #[test]
    fn test_command_registry_unknown_command() {
        let registry = CommandRegistry::new();
        let unknown_command = registry.get_command('x');

        assert!(
            unknown_command.is_none(),
            "There should be no command registered for 'x'"
        );
    }

    #[test]
    fn test_command_registry_digit_command() {
        let registry = CommandRegistry::new();
        let digit_command = registry.get_command('3');

        assert!(
            digit_command.is_some(),
            "DigitCommand should be registered for '3'"
        );
    }

    #[test]
    fn test_command_execution_add_command() {
        let registry = CommandRegistry::new();
        let add_command = registry.get_command('+').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values to the stack for addition
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(2);
            ip_locked.stk.push(3);
        }

        // Execute the command
        add_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 5, "2 + 3 should equal 5");
    }

    #[test]
    fn test_command_execution_horizontal_if_command_zero() {
        let registry = CommandRegistry::new();
        let horizontal_if_command = registry.get_command('_').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push zero to the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(0);
        }

        // Execute the command
        horizontal_if_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should move right for zero)
        let ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.direction,
            Direction::Right,
            "IP should move right for 0"
        );
    }

    #[test]
    fn test_command_execution_horizontal_if_command_non_zero() {
        let registry = CommandRegistry::new();
        let horizontal_if_command = registry.get_command('_').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push non-zero to the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(1);
        }

        // Execute the command
        horizontal_if_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should move left for non-zero)
        let ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.direction,
            Direction::Left,
            "IP should move left for non-zero"
        );
    }

    #[test]
    fn test_duplicate_top_command() {
        let registry = CommandRegistry::new();
        let duplicate_command = registry.get_command(':').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push a value to the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(42);
        }

        // Execute the command
        duplicate_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (top value should be duplicated)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 42);
        assert_eq!(ip_locked.stk.pop().unwrap(), 42);
    }

    #[test]
    fn test_swap_command() {
        let registry = CommandRegistry::new();
        let swap_command = registry.get_command('\\').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values to the stack for swapping
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(1);
            ip_locked.stk.push(2);
        }

        // Execute the command
        swap_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (top two values should be swapped)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 1);
        assert_eq!(ip_locked.stk.pop().unwrap(), 2);
    }

    #[test]
    fn test_logical_not_command() {
        let registry = CommandRegistry::new();
        let not_command = registry.get_command('!').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Stack contains 0
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(0);
        }

        // Execute the command
        not_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .expect("Execution should succeed");

        // Verify the result (0 should become 1)
        {
            let mut ip_locked = ip.lock().unwrap();
            assert_eq!(ip_locked.stk.pop().unwrap(), 1, "0 should become 1");
        }

        //  tack contains a non-zero value
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(42);
        }

        // Execute the command
        not_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .expect("Execution should succeed");

        // non-zero should become 0
        {
            let mut ip_locked = ip.lock().unwrap();
            assert_eq!(ip_locked.stk.pop().unwrap(), 0, "Non-zero should become 0");
        }
    }

    #[test]
    fn test_put_command() {
        let registry = CommandRegistry::new();
        let put_command = registry.get_command('p').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // character at (1, 2)
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push('A' as usize); // v ('A')
            ip_locked.stk.push(1); // x
            ip_locked.stk.push(2); // y
        }

        // Execute the command
        put_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify 'A' is placed at (1, 2)
        let value = mock_command_grid.get_value(1, 2).unwrap();
        assert_eq!(value, 'A', "The value at (1, 2) should be 'A'");
    }

    #[test]
    fn test_get_command() {
        let registry = CommandRegistry::new();
        let get_command = registry.get_command('g').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // 'B' at (3, 4) in the grid
        mock_command_grid.set_value(3, 4, 'B').unwrap();

        // getting a character from (3, 4)
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(3); // x
            ip_locked.stk.push(4); // y
        }

        // Execute the command
        get_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify that 'B' is pushed onto the stack
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            'B' as usize,
            "The value at (3, 4) should be 'B' pushed onto the stack"
        );
    }

    #[test]
    fn test_get_command_with_grid() {
        let registry = CommandRegistry::new();
        let get_command = registry.get_command('g').unwrap();

        // 0 start index
        let initial_grid = vec![
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
            vec![' ', ' ', 'B'],
        ];
        let mock_command_grid = MockCommandGrid::from_grid(initial_grid);

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_io_handler = Arc::new(MockIOHandler);

        // pseudo stack for get (x, y)
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(2); // X position
            ip_locked.stk.push(2); // Y position
        }

        // Execute the command
        get_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should have pushed 'B' as its ASCII value onto the stack)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            'B' as usize,
            "Value at (2, 2) should be 'B'"
        );
    }
    #[test]
    fn test_subtract_command() {
        let registry = CommandRegistry::new();
        let subtract_command = registry.get_command('-').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values to the stack for subtraction
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(5);
            ip_locked.stk.push(3);
        }

        // Execute the command
        subtract_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 2, "5 - 3 should equal 2");
    }

    #[test]
    fn test_divide_command() {
        let registry = CommandRegistry::new();
        let divide_command = registry.get_command('/').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values to the stack for division
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(10);
            ip_locked.stk.push(2);
        }

        // Execute the command
        divide_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 5, "10 / 2 should equal 5");
    }

    #[test]
    fn test_divide_command_by_zero() {
        let registry = CommandRegistry::new();
        let divide_command = registry.get_command('/').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values to the stack for division (division by zero)
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(10);
            ip_locked.stk.push(0);
        }

        // Execute the command
        divide_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (division by zero should push 0 as a fallback)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "10 / 0 should fallback to 0"
        );
    }
    #[test]
    fn test_duplicate_top_command_with_empty_stack() {
        let registry = CommandRegistry::new();
        let duplicate_command = registry.get_command(':').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Execute the command with an empty stack
        duplicate_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push 0 onto the stack when the stack is empty)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "With an empty stack, DuplicateTopCommand should push 0"
        );
    }

    #[test]
    fn test_duplicate_top_command_with_non_empty_stack() {
        let registry = CommandRegistry::new();
        let duplicate_command = registry.get_command(':').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push a value onto the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(42);
        }

        // Execute the command
        duplicate_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should duplicate the top value)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            42,
            "The top value should be duplicated"
        );
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            42,
            "The original value should still be present after duplication"
        );
    }

    #[test]
    fn test_swap_command_with_empty_stack() {
        let registry = CommandRegistry::new();
        let swap_command = registry.get_command('\\').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Execute the command with an empty stack
        swap_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push two zeros onto the stack when the stack is empty)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "With an empty stack, SwapCommand should push 0"
        );
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "With an empty stack, SwapCommand should push another 0"
        );
    }

    #[test]
    fn test_swap_command_with_one_element_stack() {
        let registry = CommandRegistry::new();
        let swap_command = registry.get_command('\\').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push a single value onto the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(42);
        }

        // Execute the command
        swap_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push a zero and keep the original value)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 0, "[42] > [42 0] > 0");
        assert_eq!(ip_locked.stk.pop().unwrap(), 42, "[42 0] > 42");
    }

    #[test]
    fn test_swap_command_with_two_elements() {
        let registry = CommandRegistry::new();
        let swap_command = registry.get_command('\\').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push two values onto the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(1);
            ip_locked.stk.push(2);
        }

        // Execute the command
        swap_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should swap the two top values)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(ip_locked.stk.pop().unwrap(), 1, "The top value should be 1");
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            2,
            "The next value should be 2"
        );
    }
    #[test]
    fn test_greater_than_command_with_greater_value() {
        let registry = CommandRegistry::new();
        let greater_than_command = registry.get_command('`').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values onto the stack where the first value is greater than the second
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(5);
            ip_locked.stk.push(3);
        }

        // Execute the command
        greater_than_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push 1 onto the stack)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            1,
            "5 > 3, so the result should be 1"
        );
    }

    #[test]
    fn test_greater_than_command_with_equal_value() {
        let registry = CommandRegistry::new();
        let greater_than_command = registry.get_command('`').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push equal values onto the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(3);
            ip_locked.stk.push(3);
        }

        // Execute the command
        greater_than_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push 0 onto the stack)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "3 is not greater than 3, so the result should be 0"
        );
    }

    #[test]
    fn test_greater_than_command_with_lesser_value() {
        let registry = CommandRegistry::new();
        let greater_than_command = registry.get_command('`').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push values onto the stack where the first value is less than the second
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(2);
            ip_locked.stk.push(3);
        }

        // Execute the command
        greater_than_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push 0 onto the stack)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "2 < 3, so the result should be 0"
        );
    }

    #[test]
    fn test_greater_than_command_with_empty_stack() {
        let registry = CommandRegistry::new();
        let greater_than_command = registry.get_command('`').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Execute the command with an empty stack
        greater_than_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push 0 onto the stack if there aren't enough elements)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "With an empty stack, the result should be 0"
        );
    }

    #[test]
    fn test_greater_than_command_with_one_element_stack() {
        let registry = CommandRegistry::new();
        let greater_than_command = registry.get_command('`').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_command_grid = MockCommandGrid::new();
        let mock_io_handler = Arc::new(MockIOHandler);

        // Push one value onto the stack
        {
            let mut ip_locked = ip.lock().unwrap();
            ip_locked.stk.push(5);
        }

        // Execute the command
        greater_than_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push 0 onto the stack if there aren't enough elements)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            0,
            "With a single-element stack, the result should be 0"
        );
    }
    #[test]
    fn test_read_number_command() {
        let registry = CommandRegistry::new();
        let read_number_command = registry.get_command('&').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_io_handler = Arc::new(MockIOHandlerWithNumber::new(42));
        let mock_command_grid = MockCommandGrid::new();

        // Execute the command
        read_number_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push the number 42 onto the stack)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            42,
            "ReadNumberCommand should push 42 onto the stack"
        );
    }

    #[test]
    fn test_read_character_command() {
        let registry = CommandRegistry::new();
        let read_character_command = registry.get_command('~').unwrap();

        // Mock objects
        let ip = Arc::new(Mutex::new(IPState::new(0, 0, Direction::Right)));
        let mock_io_handler = Arc::new(MockIOHandlerWithChar::new('A'));
        let mock_command_grid = MockCommandGrid::new();

        // Execute the command
        read_character_command
            .execute(ip.clone(), &mock_command_grid, mock_io_handler.clone())
            .unwrap();

        // Verify the result (should push the ASCII value of 'A' onto the stack)
        let mut ip_locked = ip.lock().unwrap();
        assert_eq!(
            ip_locked.stk.pop().unwrap(),
            'A' as usize,
            "ReadCharacterCommand should push ASCII value of 'A' onto the stack"
        );
    }
}
