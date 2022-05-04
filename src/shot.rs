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

use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn died(&self) -> bool {
        let out_of_bounds = self.y <= 0;
        let did_explode = self.exploding && self.timer.ready;
        out_of_bounds || did_explode
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        let shot = if self.exploding { "◉" } else { "◦" };
        frame[self.x][self.y] = shot;
    }
}
