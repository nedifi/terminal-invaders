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

// Provides modules for the terminal_invader crate.
pub mod frame;
pub mod invaders;
pub mod overlay;
pub mod player;
pub mod render;
pub mod shot;

// Provides default constants for the terminal_invader application.
pub const NUM_SHOTS: usize = 9;
