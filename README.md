# BeFungibleLab

**BeFungibleLab** is a custom, extensible Befunge interpreter written in Rust. This project allows you to explore, expand, and modify the Befunge language, making it a flexible tool for experimentation and personal use.

## Features

-  **Extensibility**: Easily extend the Befunge language with custom commands.
-  **Threading Support**: Utilizes threading for executing multiple instruction pointers (IPs).
-  **Step Mode**: Debug your Befunge programs step-by-step to better understand their execution.
-  **Grid and Stack Manipulation**: Built-in commands for interacting with the Befunge 2D grid and stack.

## Supported Commands

BeFungibleLab supports a variety of commands based on the Befunge language. Here is a list of supported commands and their descriptions:

| Command | Description                                                                       |
| ------- | --------------------------------------------------------------------------------- |
| `+`     | Adds the top two values on the stack.                                             |
| `-`     | Subtracts the top value from the second top value on the stack.                   |
| `*`     | Multiplies the top two values on the stack.                                       |
| `/`     | Divides the second top value by the top value on the stack.                       |
| `%`     | Computes the modulus of the second top value by the top value on the stack.       |
| `:`     | Duplicates the top value on the stack.                                            |
| `\\`    | Swaps the top two values on the stack.                                            |
| `$`     | Discards the top value on the stack.                                              |
| `.`     | Pops a value from the stack and prints it as a number.                            |
| `,`     | Pops a value from the stack and prints it as a character.                         |
| `@`     | Terminates the program.                                                           |
| `t`     | Pop `dy`, `dx` Creates a new thread starting from the current IP with `(dx, dy)`. |
| `!`     | Logical NOT: Pops a value and pushes `1` if zero, `0` otherwise.                  |
| `_`     | Horizontal IF: Pops a value; moves right if zero, left otherwise.                 |
| \|      | Vertical IF: Pops a value; moves down if zero, up otherwise.                      |
| `>`     | Moves the instruction pointer right.                                              |
| `<`     | Moves the instruction pointer left.                                               |
| `^`     | Moves the instruction pointer up.                                                 |
| `v`     | Moves the instruction pointer down.                                               |
| `p`     | Pops `y`, `x`, and a character, and places the character at `(x, y)`.             |
| `g`     | Pops `y` and `x`, and pushes the character at `(x, y)` onto the stack.            |
| `"`     | Toggles string mode. In string mode, each character is pushed onto the stack.     |
| `&`     | Reads an integer from input and pushes it onto the stack.                         |
| `~`     | Reads a character from input and pushes its ASCII value onto the stack.           |
| `` ` `` | Pops two values; pushes `1` if the second is greater than the first, else `0`.    |
| `0-9`   | Pushes the corresponding digit onto the stack.                                    |
| `→`     | Sets the initial direction of the instruction pointer (IP) to right.              |
| `←`     | Sets the initial direction of the instruction pointer (IP) to left.               |
| `↑`     | Sets the initial direction of the instruction pointer (IP) to up.                 |
| `↓`     | Sets the initial direction of the instruction pointer (IP) to down.               |

### Direction Commands (unstable)

The direction commands `→`, `←`, `↑`, and `↓` are used to specify the initial movement direction of an instruction pointer (IP) when the program starts. These commands determine the direction in which the IP will begin navigating the code grid:

-  **`→`**: If an IP starts at a position where this character is present, it will move right across the grid.
-  **`←`**: If an IP starts at a position where this character is present, it will move left across the grid.
-  **`↑`**: If an IP starts at a position where this character is present, it will move upwards across the grid.
-  **`↓`**: If an IP starts at a position where this character is present, it will move downwards across the grid.

If no initial direction command is specified, the default direction of the IP is set to move `→` (right).

### Example

By placing one of these direction characters (`→`, `←`, `↑`, `↓`) in your Befunge code grid, you can control the initial direction of execution. This is particularly useful for designing programs that require a specific starting flow direction.

## Installation

To get started with BeFungibleLab, you'll need to have Rust installed. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/BeFungibleLab.git
cd BeFungibleLab
cargo build --release
```

## Usage

### Running a Befunge Program

You can run a Befunge program with BeFungibleLab by passing a file path to the Befunge code as an argument. You can also enable step mode or debug mode.

```bash
cargo run --release -- path_to_your_program.bf [--step] [--debug]
```

-  `--step`: Runs the program step by step, allowing you to see each instruction as it's executed.
-  `--debug`: Displays detailed information about the state of the grid and stack after each instruction.

### Example

```bash
cargo run --release -- examples/hello_world.bf --debug
```

This will run the `hello_world.bf` Befunge program in debug mode.

### Custom Commands

BeFungibleLab allows you to add custom commands to the Befunge language. You can do this by modifying the command registry and implementing the desired logic for each new command. This extensibility makes it easy to experiment with new features and instructions.

## Development

Feel free to fork this repository and make your own modifications! Here's how you can get started with development:

1. Clone the repo:

   ```bash
   git clone https://github.com/yourusername/BeFungibleLab.git
   cd BeFungibleLab
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the tests:
   ```bash
   cargo test
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
