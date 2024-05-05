use clap::{Parser, Subcommand};
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{ConnectionExt, GrabMode, GrabStatus},
        Event,
    },
    NONE,
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

            loop {
                let event = conn.wait_for_event()?;

                match event {
                    Event::KeyRelease(event) => {
                        let pointer_query = conn.query_pointer(screen.root)?.reply()?;
                        let (x, y) = (pointer_query.root_x, pointer_query.root_y);
                        match event.detail {
                            9 /* Escape */ => break,
                            43 /* h */ => {
                                conn.warp_pointer(NONE, screen.root, 0, 0, 0, 0, x - x_offset, y)?;
                            }
                            44 /* j */ => {
                                conn.warp_pointer(NONE, screen.root, 0, 0, 0, 0, x, y + y_offset)?;
                            }
                            45 /* k */ => {
                                conn.warp_pointer(NONE, screen.root, 0, 0, 0, 0, x, y - y_offset)?;
                            }
                            46 /* l */ => {
                                conn.warp_pointer(NONE, screen.root, 0, 0, 0, 0, x + x_offset, y)?;
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
                            58 /* m */ => {}
                            59 /* , */ => {}
                            60 /* . */ => {}
                            key => {
                                println!("Pressed key: {key:?}");
                            }
                        }
                    }
                    _ => {}
                }

                conn.flush()?;
            }

            conn.ungrab_keyboard(x11rb::CURRENT_TIME)?;
        }
    };

    Ok(())
}
