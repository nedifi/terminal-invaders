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

// Provides a struct for shots with coordinates and explosion attributes.
pub struct Shot {
    pub x: u16,
    pub y: u16,
    pub exploding: bool,
    timer: Timer,
}

// Implements the shot struct.
impl Shot {

    // Creates a new shot at the given position.
    pub fn new(x: u16, y: u16) -> Self {
        Self {

            // Fires the shot at x, y.
            x,
            y,

            // It's not exploding, yet.
            exploding: false,

            // It moves with a speed of 1/50ms.
            timer: Timer::from_millis(50),
        }
    }

    // Updates the shot's positions.
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);

        // Only moves the shot if the timer is ready.
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    // Explodes the shot.
    pub fn explode(&mut self) {
        self.exploding = true;

        // Extends explosion time.
        self.timer = Timer::from_millis(250);
    }

    // Determines whether a shot died.
    pub fn died(&self) -> bool {

        // Is it out of bounds?
        let out_of_bounds = self.y <= 0;

        // Did it explode?
        let did_explode = self.exploding && self.timer.ready;
        out_of_bounds || did_explode
    }
}

// Implements the drawable trait for shots.
impl Drawable for Shot {

    // Draws a shot on a given frame.
    fn draw(&self, frame: &mut Frame) {

        // Animates explosions.
        let shot = if self.exploding { "◉" } else { "◦" };
        frame[self.x as usize][self.y as usize] = shot.to_string();
    }
}
