// Copyright 2021-2022 @nedifi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::frame::Frame;

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::Stdout;

// Renders the current frame over the last frame.
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // Forces the entire frame canvas to reset.
    if force {
        stdout
            .queue(SetBackgroundColor(Color::Grey))
            .expect("Background color should be set to grey!");
        stdout
            .queue(Clear(ClearType::All))
            .expect("It should clear the entire STDOUT.");
        stdout
            .queue(SetBackgroundColor(Color::Black))
            .expect("Background color should be set to black!");
    }

    // Iterates all colums over x coordinates.
    for (x, col) in curr_frame.iter().enumerate() {
        // Iterates all characters of y coordinates.
        for (y, s) in col.iter().enumerate() {
            // Moves the queue to the requested position of x, y.
            stdout.queue(MoveTo(x as u16, y as u16)).expect(&format!(
                "It shoud move to the position {}, {}.",
                x.to_string(),
                y.to_string()
            ));

            // Updates the frame if the character changed.
            if *s != last_frame[x][y] || force {
                print!("{}", s);
            }
        }
    }
}
