use clap::{Parser, Subcommand};
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

            loop {
                let event = conn.wait_for_event()?;

                match event {
                    Event::KeyRelease(event) => match event.detail {
                        9 => break,
                        key => {
                            println!("Released key: {key:?}");
                        }
                    },
                    _ => {}
                }
            }

            conn.ungrab_keyboard(x11rb::CURRENT_TIME)?;
        }
    };

    Ok(())
}
