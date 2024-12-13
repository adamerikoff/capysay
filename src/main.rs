use clap::{Parser, Subcommand};
use colored::Colorize;
use cursive::event::Key;
use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, EditView, ListView};
use cursive::Cursive;

#[derive(Parser)]
#[command(name = "capysay", version = "0.3.0", about = "A Rust-based CLI tool for customizable Capybara ASCII art with colorful messages!")]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Capysay in default mode
    Default {
        #[arg(default_value = "Libenter homines id quod volunt credunt!")]
        /// What should Capybara say?
        message: String,
        #[arg(short = 'd', long = "dead")]
        /// Make the Capybara appear dead.
        dead: bool,
    },
    /// Run Capysay in TUI mode
    Tui {
        #[arg(default_value = "Libenter homines id quod volunt credunt!")]
        /// What should Capybara say?
        message: String,
        #[arg(short = 'd', long = "dead")]
        /// Make the Capybara appear dead.
        dead: bool,
    },
}

fn run_default_mode(message: String, dead: bool) {
    let eye = if dead { "X" } else { "⠛" };

    if message.to_lowercase() == "meow" {
        eprintln!("Capybara doesn't meow!")
    }
    println!("              {}", message.bright_yellow().underline().on_purple());
    println!("                       \\");
    println!("                        \\");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣄⢘⣒⣀⣀⣀⣀⠀⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣽⣿⣛{eye}⢛⣿⣿⡿⠟⠂⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⡀⠀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⡆⠀");
    println!("⠀⠀⠀⠀⠀⠀⣀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀");
    println!("⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠜⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠀⢿⣿⣿⣿⣿⠿⠿⣿⣿⡿⢿⣿⣿⠈⣿⣿⣿⡏⣠⡴⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⣠⣿⣿⣿⡿⢁⣴⣶⣄⠀⠀⠉⠉⠉⠀⢻⣿⡿⢰⣿⡇⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⢿⣿⠟⠋⠀⠈⠛⣿⣿⠀⠀⠀⠀⠀⠀⠸⣿⡇⢸⣿⡇⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⢸⣿⠀⠀⠀⠀⠀⠘⠿⠆⠀⠀⠀⠀⠀⠀⣿⡇⠀⠿⠇⠀⠀⠀⠀⠀⠀⠀");
    println!("█─▄▄▄─██▀▄─██▄─▄▄─█▄─█─▄█─▄▄▄▄██▀▄─██▄─█─▄█");
    println!("█─███▀██─▀─███─▄▄▄██▄─▄██▄▄▄▄─██─▀─███▄─▄██");
    println!("▀▄▄▄▄▄▀▄▄▀▄▄▀▄▄▄▀▀▀▀▄▄▄▀▀▄▄▄▄▄▀▄▄▀▄▄▀▀▄▄▄▀▀");
}

#[derive(Debug)]
struct CatsayOptions {
    message: String,
    dead: bool,
}

fn tui_input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Capysay Input")
            .content(
                ListView::new()
                    .child(
                        "Message",
                        EditView::new().with_name("message"),
                    )
                    .child("Dead?", Checkbox::new().with_name("dead")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_name("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let is_dead = s
                    .call_on_name("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = CatsayOptions {
                    message: message.to_string(),
                    dead: is_dead,
                };
                tui_result_step(s, &options)
            }),
    );
}

fn tui_result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "⠛" };
    let cat_text = format!(
        "
                    {msg}
                       \\
                        \\
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣄⢘⣒⣀⣀⣀⣀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣽⣿⣛{eye}⢛⣿⣿⡿⠟⠂⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⡀⠀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⡆⠀
        ⠀⠀⠀⠀⠀⠀⣀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀
        ⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
        ⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
        ⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠜⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⢿⣿⣿⣿⣿⠿⠿⣿⣿⡿⢿⣿⣿⠈⣿⣿⣿⡏⣠⡴⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⣠⣿⣿⣿⡿⢁⣴⣶⣄⠀⠀⠉⠉⠉⠀⢻⣿⡿⢰⣿⡇⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⢿⣿⠟⠋⠀⠈⠛⣿⣿⠀⠀⠀⠀⠀⠀⠸⣿⡇⢸⣿⡇⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⢸⣿⠀⠀⠀⠀⠀⠘⠿⠆⠀⠀⠀⠀⠀⠀⣿⡇⠀⠿⠇⠀⠀⠀⠀⠀⠀⠀
        ",
        msg = options.message,
        eye = eye
    );
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(cat_text)
            .title("The Capybara says...")
            .button("OK", |s| s.quit()),
    );
}

fn run_tui_mode(_message: String, _dead: bool) {
    let mut siv = cursive::default();
    tui_input_step(&mut siv);
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Some(Commands::Default { message, dead }) => {
            run_default_mode(message, dead);
        }
        Some(Commands::Tui { message, dead }) => {
            run_tui_mode(message, dead);
        }
        None => {
            println!("Use `--help` for available commands and options.");
        }
    }
}