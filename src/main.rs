use std::{thread, time::Duration};

use lamertin::core::{Color, Direction, Lamertin};

fn _wait() {
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn _crosses_and_circles() {
    let mut lamertin = Lamertin::new().unwrap();
    let mut is_even;
    lamertin.clear_screen();
    for i in 0..100 {
        is_even = (i % 2) == 0;
        if is_even {
            lamertin.set_fg_and_bg(&Color::Green, &Color::Red);
        } else {
            lamertin.set_fg_and_bg(&Color::Black, &Color::White);
        }
        lamertin.place_char(if is_even { 'X' } else { 'O' });
        lamertin.move_dir(Direction::Down, 2);
        lamertin.move_dir(Direction::Right, 1);
        thread::sleep(Duration::from_millis(750));
    }
}

fn _placement() {
    let mut lamertin = Lamertin::new().unwrap();
    lamertin.clear_screen();
    lamertin.init_window(20, 50);
    lamertin.set_pos(10, 10);
    lamertin.place_char('X');
    thread::sleep(Duration::from_secs(2));
    lamertin.place_char_at('O', 5, 5);
    lamertin.set_fg_and_bg(&Color::Blue, &Color::Black);
    lamertin.place_str_at("Hello", 2, 4);
    lamertin.set_fg_and_bg(&Color::Red, &Color::White);
    lamertin.place_str_at("World", 3, 4);
}

fn main() {
    let mut lamertin = Lamertin::new().unwrap();
    lamertin.init_window(10, 10);
    lamertin.set_fg(&Color::Red);
    lamertin.draw_rect_at('X', 3, 5, 5, 3);
    _wait();
}
