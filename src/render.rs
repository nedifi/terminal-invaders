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
            stdout.queue(MoveTo(x as u16, y as u16)).expect(&format!(
                "It shoud move to the position {}, {}.",
                x.to_string(),
                y.to_string()
            ));
            if *s != last_frame[x][y] || force {
                print!("{}", s);
            } else {
                print!("{}", last_frame[x][y]);
            }
        }
    }
}
