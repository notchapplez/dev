use std::fmt::Alignment::Left;
use clap::{Parser, Arg, Command, Subcommand};
use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(version, about, long_about = None, name = "dev")]
struct Args {
    #[command(subcommand)]
     action: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Note {
        note_text: String
    },
    Femboy,
    Placeholder,
    /* grep, jokes, utils, cat, etc. (toolbox, i know, i know) */
}

fn main() {
    let args = Args::parse();

    match &args.action {
        Commands::Note { note_text } => println!("{}: {}", "Note".bright_blue(), note_text),
        Commands::Femboy => println!("{}", "Femboy :3".purple().bold().underline()), //At this point my IntelliSense trolled me by suggesting "Commands::Femboy => println!("{}: {}", "Femboy".bright_blue(), "https://www.youtube.com/watch?v=dQw4w9WgXcQ".bright_blue()),"
        _ => println!("No command found")
    }
}