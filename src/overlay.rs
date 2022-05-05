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

// Provides an overlay with customizable strings.
pub struct Overlay {
    timer: Timer,
    lines: Vec<String>,
}

// Implements the overlay.
impl Overlay {

    // Creates a new overlay.
    pub fn new() -> Self {
        Self {

            // Provides a timer updating the overlay every 100ms.
            timer: Timer::from_millis(100),

            // Creates an empty string collection.
            lines: vec![" ".to_string(); 2],
        }
    }

    // Updates the overlay with debugging information.
    pub fn update(&mut self, delta: Duration, dimensions: &Vec<u16>) {
        self.timer.update(delta);
        if self.timer.ready {
            let fps = 1_f64 / delta.as_secs_f64();
            let lag = (delta.as_nanos() as f64 - 5_000_000_f64) / 1_000_000_f64;
            self.lines[0] = format!(
                "dimension: {:?} cols x {:?} rows",
                dimensions[0], dimensions[1]
            );
            self.lines[1] = format!(
                "delta: {:.5}s; render(): {:.2}/s; duration: {:5}ms",
                delta.as_secs_f64(),
                fps,
                lag
            );
            self.timer.reset();
        }
    }
}

// Implements the drawable trait for the overlay.
impl Drawable for Overlay {

    // Draws the overlay on a given frame.
    fn draw(&self, frame: &mut Frame) {
        frame[0][0] = self.lines[0].clone();
        frame[0][1] = self.lines[1].clone();
    }
}
