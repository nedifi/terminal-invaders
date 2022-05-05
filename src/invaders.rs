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

use std::{cmp::max, time::Duration};

use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame};

// Provides an invader struct with coordinates.
pub struct Invader {
    x: u16,
    y: u16,
}

// Provides an invaders struct for an entire army.
pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
    bounds: Vec<u16>,
}

// Implements the invaders army.
impl Invaders {

    // Creates a new invaders army within the given dimensions.
    pub fn new(dimensions: &Vec<u16>) -> Self {

        // Creates the army.
        let mut army = Vec::new();
        for x in 0..dimensions[0] {
            for y in 0..dimensions[1] {
                if (x > 1)
                    && (x < dimensions[0] - 2)
                    && (y > 0)
                    && (y < dimensions[1] / 2 - 2)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }
        Self {
            army,

            // The army changes position every 2 seconds.
            move_timer: Timer::from_millis(2000),

            // The army moves in different directions.
            direction: 1,

            // The army's boundaries.
            bounds: dimensions.clone(),
        }
    }

    // Updates the army positions.
    pub fn update(&mut self, delta: Duration) {
        self.move_timer.update(delta);

        // Only move if timer is ready.
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;

            // Determines wether it's time to move downwards.
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == self.bounds[0] - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            // Moves downwards.
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as u16;
                }
            }
        }
    }

    // Determines whether all invaders were killed.
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    // Determines whether the invaders reached the bottom.
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= self.bounds[1] - 1
    }

    // Kills an invader at the given position.
    pub fn kill_invader_at(&mut self, x: u16, y: u16) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

// Implements the drawable trait for the invaders army.
impl Drawable for Invaders {

    // Draws the invaders on a given frame.
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {

            // Allows invaders to change appearance.
            frame[invader.x as usize][invader.y as usize] =
                if (self.move_timer.time_left.as_secs_f32()
                    / self.move_timer.duration.as_secs_f32())
                    > 0.5
                {
                    "■".to_string()
                } else {
                    "□".to_string()
                }
        }
    }
}
