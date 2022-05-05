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

use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
    NUM_SHOTS,
};

// Provides a player struct with coordinates, bounds, and shots.
pub struct Player {
    x: u16,
    y: u16,
    bounds: Vec<u16>,
    shots: Vec<Shot>,
}

// Implements the player struct.
impl Player {

    // Creates a new player within the given boundaries.
    pub fn new(dimensions: &Vec<u16>) -> Self {
        Self {

            // Centers the player on the bottom of the terminal.
            x: dimensions[0] / 2,
            y: dimensions[1] - 1,
            bounds: dimensions.clone(),

            // Prepares the shots for the player.
            shots: Vec::new(),
        }
    }

    // Moves the player position left.
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    // Moves the player position right.
    pub fn move_right(&mut self) {
        if self.x < self.bounds[0] - 1 {
            self.x += 1;
        }
    }

    // Moves the player position up.
    pub fn move_up(&mut self) {
        if self.y > self.bounds[1] / 2 {
            self.y -= 1;
        }
    }

    // Moves the player position down.
    pub fn move_down(&mut self) {
        if self.y < self.bounds[1] - 1 {
            self.y += 1;
        }
    }

    // Shoots at the invaders.
    pub fn shoot(&mut self) {

        // We only have a limit of `NUM_SHOTS`.
        if self.shots.len() <= NUM_SHOTS {
            self.shots.push(Shot::new(self.x, self.y - 1));
        }
    }

    // Updates the player position.
    pub fn update(&mut self, delta: Duration) {

        // Updates the shot positions.
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        // Only retain the shots that are still alive.
        self.shots.retain(|shot| !shot.died());
    }

    // Detects a hit of a shot and an invader.
    pub fn detect_hit(&mut self, invaders: &mut Invaders) {
        for shot in self.shots.iter_mut() {

            // Only non-exploding shots can hit invaders.
            if !shot.exploding {

                // Tries to kill an invader at the current position.
                if invaders.kill_invader_at(shot.x, shot.y) {

                    // Explodes if invader is killed.
                    shot.explode();
                }
            }
        }
    }
}

// Implements the drawable trait for the player.
impl Drawable for Player {

    // Draws the player and shots on the given frame.
    fn draw(&self, frame: &mut Frame) {
        frame[self.x as usize][self.y as usize] = "△".to_string();
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
