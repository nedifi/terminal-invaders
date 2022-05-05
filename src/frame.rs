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

// Provides a frame canvas as two-dimensional collection.
pub type Frame = Vec<Vec<String>>;

// Prepares an empty two-dimensional frame to be used to draw on.
pub fn new_frame(dimensions: &Vec<u16>) -> Frame {
    let x = dimensions[0];
    let y = dimensions[1];
    let mut cols = Vec::with_capacity(x.into());
    for _ in 0..x {
        let mut row = Vec::with_capacity(y.into());
        for _ in 0..y {
            row.push(" ".to_string());
        }
        cols.push(row);
    }
    cols
}

// Provides a drawable trait for all structs to be displayed on the frame.
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
