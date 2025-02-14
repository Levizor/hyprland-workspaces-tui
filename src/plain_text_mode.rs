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
        "{}{}{}",
        if config.carriage_return { "\r" } else { "" },
        message,
        if config.carriage_return { "" } else { "\n" },
    );
    io::stdout().flush().unwrap();
    log::info!("MSG: {}", message);
}

pub async fn main(app: &mut App) -> AppResult<()> {
    let config = if let Some(Commands::Plain {
        separator,
        active,
        carriage_return,
        print_once,
    }) = &app.config.command
    {
        PlainConfig {
            separator: separator.into(),
            active: active.into(),
            carriage_return: *carriage_return,
            print_once: *print_once,
        }
    } else {
        PlainConfig {
            separator: String::from(" "),
            active: String::from("|"),
            carriage_return: false,
            print_once: false,
        }
    };

    while app.running {
        tokio::select! {
            Ok(Some(line)) = app.stream.next_line() => {
                app.feed(line);
            }

            _ = signal::ctrl_c() => {
                println!();
                app.quit().await;
            }
        }
        print(&app, &config);
        if config.print_once {
            println!();
            app.quit().await;
        }
    }

    Ok(())
}

struct PlainConfig {
    separator: String,
    active: String,
    carriage_return: bool,
    print_once: bool,
}
