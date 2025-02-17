use crate::{app::AppResult, App};
use std::io::{self, Write};
use tokio::signal;

fn print(app: &App) {
    let config = &app.config.plain_text_mode;

    let mut message = String::new();
    let ws = &app.workspaces;
    let len = ws.len();
    for i in 0..len {
        if ws[i].is_active() {
            message.push_str(&format!("{1}{0}{1}", ws[i].name, config.separator_active));
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
        print(&app);
        if app.config.plain_text_mode.print_once {
            println!();
            app.quit().await;
        }
    }

    Ok(())
}
