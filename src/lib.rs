use std::io::{self, Stdout, Write};

struct Cursor;

impl Cursor {
    pub fn move_up() {
        todo!()
    }

    pub fn move_up_by(n: u8) {
        todo!()
    }

    pub fn move_down() {
        todo!()
    }

    pub fn move_down_by(n: u8) {
        todo!()
    }

    pub fn move_right() {
        todo!()
    }

    pub fn move_right_by() {
        todo!()
    }

    pub fn move_left() {
        todo!()
    }

    pub fn move_left_by() {
        todo!()
    }

    fn flush_stdout() {
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
