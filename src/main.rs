use clap::{Parser, Subcommand};
use enigo::{Axis, Button, Coordinate, Direction::Click, Enigo, Mouse, Settings};
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{ConnectionExt, GrabMode, GrabStatus},
        Event,
    },
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run in normal mode.
    Normal,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Normal => {
            let (conn, screen_num) = x11rb::connect(None)?;
            let screen = &conn.setup().roots[screen_num];

            let reply = conn
                .grab_keyboard(
                    true,
                    screen.root,
                    x11rb::CURRENT_TIME,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                )?
                .reply()?;
            if reply.status == GrabStatus::SUCCESS {
                println!("Grabbed keyboard successfully!");
            } else {
                println!("Failed to Grab keyboard!");
            }

            let (mut x_offset, mut y_offset) = (20, 20);
            let scroll_offset = 5;
            let mut held_shift = false;

            let mut enigo = Enigo::new(&Settings::default())?;
            loop {
                let event = conn.wait_for_event()?;

                match event {
                    Event::KeyPress(event) => {
                        match event.detail {
                            9 /* Escape */ => break,
                            43 /* h */ => {
                                if held_shift {
                                    enigo.scroll(-scroll_offset, Axis::Horizontal)?;
                                } else {
                                    enigo.move_mouse(-x_offset, 0, Coordinate::Rel)?;
                                }
                            }
                            44 /* j */ => {
                                if held_shift {
                                    enigo.scroll(scroll_offset, Axis::Vertical)?;
                                } else {
                                    enigo.move_mouse(0, y_offset, Coordinate::Rel)?;
                                }
                            }
                            45 /* k */ => {
                                if held_shift {
                                    enigo.scroll(-scroll_offset, Axis::Vertical)?;
                                } else {
                                    enigo.move_mouse(0, -y_offset, Coordinate::Rel)?;
                                }
                            }
                            46 /* l */ => {
                                if held_shift {
                                    enigo.scroll(scroll_offset, Axis::Horizontal)?;
                                } else {
                                    enigo.move_mouse(x_offset, 0, Coordinate::Rel)?;
                                }
                            }
                            38 /* a */ => {
                                x_offset += 10;
                                if x_offset < 0 {
                                    x_offset = 20;
                                }
                                y_offset = x_offset;
                            }
                            40 /* d */ => {
                                x_offset -= 10;
                                if x_offset < 0 {
                                    x_offset = 20;
                                }
                                y_offset = x_offset;
                            }
                            58 /* m */ => {
                                enigo.button(Button::Left, Click)?;
                            }
                            59 /* , */ => {
                                enigo.button(Button::Middle, Click)?;
                            }
                            60 /* . */ => {
                                enigo.button(Button::Right, Click)?;
                            }
                            50 /* shift */ => {
                                held_shift = true;
                            }
                            key => {
                                println!("Pressed key: {key:?}");
                            }
                        }
                    }
                    Event::KeyRelease(event) => {
                        match event.detail {
                            50 /* shift */ => {
                                held_shift = false;
                            }
                            key => {
                                println!("Released key: {key:?}");
                            }
                        }
                    }
                    _ => (),
                }

                conn.flush()?;
            }

            conn.ungrab_keyboard(x11rb::CURRENT_TIME)?;
        }
    };

    Ok(())
}
