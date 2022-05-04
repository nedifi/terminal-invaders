use crate::frame::Frame;

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::Stdout;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout
            .queue(SetBackgroundColor(Color::Blue))
            .expect("Background color should be set to blue!");
        stdout
            .queue(Clear(ClearType::All))
            .expect("It should clear the entire STDOUT.");
        stdout
            .queue(SetBackgroundColor(Color::Black))
            .expect("Background color should be set to black!");
    }
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).expect(&format!(
                    "It shoud move to the position {}, {}.",
                    x.to_string(),
                    y.to_string()
                ));
                print!("{}", s);
            }
        }
    }
}
