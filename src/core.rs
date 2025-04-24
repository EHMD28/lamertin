use std::{
    cmp::{max, min},
    io::{self, Stdout, Write},
    os::fd::AsRawFd,
};

use termios::Termios;

pub struct Palette {
    fg: Color,
    bg: Color,
}

#[derive(Clone, Copy)]
struct Position {
    row: u8,
    column: u8,
}

pub struct Lamertin {
    stdout: Stdout,
    termios: Termios,
    palette: Palette,
    position: Position,
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    None,
}

impl Lamertin {
    /// Creates a new instance of a `Lamertin` object which is used for interacting with the
    /// terminal. This object keeps track of current state, including colors and position.
    pub fn new() -> io::Result<Self> {
        let fd = std::io::stdin().as_raw_fd();
        let termios = Termios::from_fd(fd)?;
        Ok(Self {
            stdout: io::stdout(),
            termios,
            palette: Palette {
                fg: Color::None,
                bg: Color::None,
            },
            position: Position { row: 1, column: 1 },
        })
    }

    pub fn fg_color(&self) -> &Color {
        &self.palette.fg
    }

    pub fn bg_color(&self) -> &Color {
        &self.palette.bg
    }

    pub fn set_echo(&mut self, on: bool) {
        todo!()
    }

    pub fn get_char(&mut self) -> char {
        todo!()
    }

    pub fn clear_screen(&mut self) {
        print!("\x1b[1;1H\x1b[2J");
        self.stdout.flush().unwrap();
    }

    pub fn clear_screen_with_scrollback(&mut self) {
        print!("\x1b[1;1H\x1b[3J");
        self.stdout.flush().unwrap();
    }

    pub fn init_window(&mut self, rows: u8, columns: u8) {
        let row = " ".repeat(columns as usize);
        for _ in 0..rows {
            println!("{}", row);
        }
        print!("\x1b[1;1H");
        self.stdout.flush().unwrap();
    }

    pub fn move_dir(&mut self, direction: Direction, amount: u8) {
        let ending = match direction {
            Direction::Up => {
                self.position.row = max(self.position.row - 1, 1);
                'A'
            }
            Direction::Down => {
                // TODO: Change 10 to real number.
                self.position.row = min(self.position.row + 1, 10);
                'B'
            }
            Direction::Right => {
                // TODO: Change 10 to real number
                self.position.column = min(self.position.column + 1, 10);
                'C'
            }
            Direction::Left => {
                self.position.column = max(self.position.column - 1, 1);
                'D'
            }
        };
        let code = format!("\x1b[{amount}{ending}");
        let _ = self.stdout.write_all(code.as_bytes());
        let _ = self.stdout.flush();
    }

    pub fn set_pos(&mut self, row: u8, column: u8) {
        self.position.row = row;
        self.position.column = column;
        print!("\x1b[{row};{column}H");
        self.stdout.flush().unwrap();
    }

    pub fn place_char(&mut self, ch: char) {
        print!("{ch}");
        self.stdout.flush().unwrap();
    }

    pub fn place_char_at(&mut self, ch: char, row: u8, column: u8) {
        let current_pos = self.position;
        self.set_pos(row, column);
        self.place_char(ch);
        self.set_pos(current_pos.row, current_pos.column);
    }

    pub fn place_str(&mut self, s: &str) {
        print!("{s}");
        self.stdout.flush().unwrap();
    }

    pub fn place_str_at(&mut self, s: &str, row: u8, column: u8) {
        let current_pos = self.position;
        self.set_pos(row, column);
        self.place_str(s);
        self.set_pos(current_pos.row, current_pos.column);
    }

    fn color_to_ansi_fg(color: &Color) -> u32 {
        match color {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::None => 0,
        }
    }

    fn color_to_ansi_bg(color: &Color) -> u32 {
        match color {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::None => 0,
        }
    }

    fn paint_palette(&mut self) {
        let fg = Lamertin::color_to_ansi_fg(self.fg_color());
        let bg = Lamertin::color_to_ansi_bg(self.bg_color());
        print!("\x1b[{fg};{bg}m");
        self.stdout.flush().unwrap();
    }

    pub fn set_fg(&mut self, color: &Color) {
        self.palette.fg = color.to_owned();
        self.paint_palette();
    }

    pub fn set_bg(&mut self, color: &Color) {
        self.palette.bg = color.to_owned();
        self.paint_palette();
    }

    pub fn set_fg_and_bg(&mut self, fg: &Color, bg: &Color) {
        self.palette.fg = fg.to_owned();
        self.palette.bg = bg.to_owned();
        self.paint_palette();
    }
}
