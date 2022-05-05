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

use clap::{Arg, Command};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use std::error::Error;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::{io, thread};
use terminal_invaders::frame::{new_frame, Drawable, Frame};
use terminal_invaders::invaders::Invaders;
use terminal_invaders::overlay::Overlay;
use terminal_invaders::player::Player;
use terminal_invaders::{frame, render};

// Main entry point for the terminal_invader application.
fn main() -> Result<(), Box<dyn Error>> {
    // Allows to specify a debug mode.
    let args = Command::new("terminal_invaders")
        .about("Terminal game application to pass some time while compiling Rust.")
        .version("0.1.0")
        .author("@nedifi <rust@nedi.fi>")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug overlay; this might make the game unplayable."),
        )
        .get_matches();
    let debug_mode = args.is_present("debug");
    let mut game_won = false;

    // Creates a stdout for an alternative terminal in raw mode.
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Gets the terminal dimensions.
    let (dim_x, dim_y) = terminal::size().expect("Terminal should have a specific size.");
    let dimensions = vec![dim_x, dim_y];
    let render_dimensions = dimensions.clone();

    // Creates a render thread that can be fed with frames.
    let (render_tx, render_rx) = channel::<Frame>();
    let render_handle = thread::spawn(move || {
        // Renders an empty frame (forced).
        let mut last_frame = frame::new_frame(&render_dimensions);
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        // Launches a render loop that keeps listening for new frames.
        'renderloop: loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break 'renderloop,
            };

            // Render the new frame and retain it as reference for the next iteration.
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Populate the game with players, timers, and an army of terminal invaders.
    let mut overlay = Overlay::new();
    let mut player = Player::new(&dimensions);
    let mut invaders = Invaders::new(&dimensions);
    let mut instant = Instant::now();

    // Creates a game loop that listens for keyboard inputs.
    'gameloop: loop {
        // Keeps track of time.
        let delta = instant.elapsed();
        instant = Instant::now();

        // Starts with an empty frame.
        let mut curr_frame = new_frame(&dimensions);

        // Handles all key-code inputs.
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Down => player.move_down(),
                    KeyCode::Up => player.move_up(),
                    KeyCode::Char(' ') | KeyCode::Enter => player.shoot(),
                    KeyCode::Char('q') | KeyCode::Esc => break 'gameloop,
                    _ => {}
                }
            }
        }

        // Updates player and invader positions.
        player.update(delta);
        invaders.update(delta);
        overlay.update(delta, &dimensions);

        // Detects shots hitting invaders.
        player.detect_hit(&mut invaders);

        // Draws player and all remaining invaders.
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        // Draws a debug overlay if mode is enabled.
        if debug_mode {
            overlay.draw(&mut curr_frame);
        }

        // Sends the prepared frame to the render channel.
        let _ = render_tx.send(curr_frame);

        // Forces the game loop to slow down to save CPU cycles.
        thread::sleep(Duration::from_millis(5));

        // Winning condition: if invaders are all dead.
        if invaders.all_killed() {
            game_won = true;
            break 'gameloop;
        }

        // Losing condition: if invaders reached bottom.
        if invaders.reached_bottom() {
            break 'gameloop;
        }
    }

    // Cleans up the threads and terminal once the game ends.
    drop(render_tx);
    render_handle
        .join()
        .expect("The render thread should be done by now.");
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Displays results.
    if game_won {
        println!("You won!");
    } else {
        println!("Game over!")
    }

    // Shuts down normally.
    Ok(())
}
