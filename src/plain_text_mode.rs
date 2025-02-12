use crate::{app::AppResult, config::Commands, App};
use std::io::{self, Write};
use tokio::signal;

fn print(app: &App, config: &PlainConfig) {
    let mut message = String::new();
    let ws = &app.workspaces;
    let len = ws.len();
    for i in 0..len {
        if ws[i].is_active() {
            message.push_str(&format!("{1}{0}{1}", ws[i].name, config.active));
        } else {
            message.push_str(&format!("{}", ws[i].name));
        }

        if i != len - 1 {
            message.push_str(&config.separator);
        }
    }
    print!(
        "{}{}",
        if config.carriage_return { "\r" } else { "\n" },
        message
    );
    io::stdout().flush().unwrap();
    log::info!("MSG: {}", message);
}

pub async fn main(app: &mut App) -> AppResult<()> {
    let config = if let Some(Commands::Plain {
        separator,
        active,
        carriage_return,
    }) = &app.config.command
    {
        PlainConfig {
            separator: separator.clone(),
            active: active.clone(),
            carriage_return: carriage_return.clone(),
        }
    } else {
        PlainConfig {
            separator: String::from(" "),
            active: String::from("|"),
            carriage_return: true,
        }
    };

    while app.running {
        print(&app, &config);
        tokio::select! {
            Ok(Some(line)) = app.stream.next_line() => {
                app.feed(line);
            }

            _ = signal::ctrl_c() => {
                println!();
                app.quit();
            }
        }
    }

    Ok(())
}

struct PlainConfig {
    separator: String,
    active: String,
    carriage_return: bool,
}
