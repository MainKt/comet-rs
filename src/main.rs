use clap::{Parser, Subcommand};
use enigo::{Button, Coordinate, Direction::Click, Enigo, Mouse, Settings};
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

            let (mut x_offset, mut y_offset) = (10, 10);

            let mut enigo = Enigo::new(&Settings::default())?;
            loop {
                let event = conn.wait_for_event()?;

                if let Event::KeyRelease(event) = event {
                    match event.detail {
                        9 /* Escape */ => break,
                        43 /* h */ => {
                            enigo.move_mouse(-x_offset, 0, Coordinate::Rel)?;
                        }
                        44 /* j */ => {
                            enigo.move_mouse(0, y_offset, Coordinate::Rel)?;
                        }
                        45 /* k */ => {
                            enigo.move_mouse(0, -y_offset, Coordinate::Rel)?;
                        }
                        46 /* l */ => {
                            enigo.move_mouse(x_offset, 0, Coordinate::Rel)?;
                        }
                        38 /* a */ => {
                            x_offset += 10;
                            if x_offset < 0 {
                                x_offset = 10;
                            }
                            y_offset = x_offset;
                        }
                        40 /* d */ => {
                            x_offset -= 10;
                            if x_offset < 0 {
                                x_offset = 10;
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
                        key => {
                            println!("Pressed key: {key:?}");
                        }
                    }
                }

                conn.flush()?;
            }

            conn.ungrab_keyboard(x11rb::CURRENT_TIME)?;
        }
    };

    Ok(())
}
