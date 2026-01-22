use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::io;
use std::io::{BufRead, Read};
use sysinfo;

#[derive(Parser)]
#[command(version, about, long_about = None, name = "dev")]
struct Args {
    #[command(subcommand)]
    action: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Note { note_text: String },
    Femboy,
    Placeholder,
    Info,
    Cat { femboy: Option<String> },
    Grep { search: String }, /* grep, jokes, utils, cat, etc. (toolbox, I know, I know) */
}

fn cat(femboy_name: Option<String>) {
    if femboy_name.is_none() {
        loop {
            let mut femput = String::new();
            io::stdin().read_line(&mut femput).expect("Failed to read from fembin");
            print!("{}", femput);
        }
    }

    let contents = std::fs::read_to_string(&femboy_name.unwrap()).expect("Something went wrong reading the femboy");
    println!("{}", contents);
}

pub fn grep(search_pattern: String) {
    let mut stdin = String::new();
    io::stdin()
        .read_to_string(&mut stdin)
        .expect("Failed to read from stdin");

    let string = stdin.trim().lines().collect::<Vec<&str>>();

    let results: Vec<&str> = string
        .into_iter()
        .filter(|line| line.contains(&search_pattern))
        .collect();

    for result in &results {
        let colored_line =
            result.replace(&search_pattern, &search_pattern.red().bold().to_string());
        println!("{}", colored_line);
    }
}

fn gather() {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    println!("\n\n              CPU Usage: {}%", system.global_cpu_usage().red());
    println!("              Total Memory: {}", system.total_memory().red());
    println!("              Free Memory: {}", system.free_memory().red());
    println!("              Total Swap: {}", system.total_swap().red());
    println!("              Free Swap: {}", system.free_swap().red());
    println!("              Processes: {}\n\n", system.processes().len().red());
}

fn main() {
    let args = Args::parse();

    match &args.action {
        Commands::Note { note_text } => println!("{}: {}", "Note".bright_blue(), note_text),
        Commands::Femboy => println!("{}", "Femboy :3".purple().bold().underline()), //At this point my IntelliSense trolled me by suggesting "Commands::Femboy => println!("{}: {}", "Femboy".bright_blue(), "https://www.youtube.com/watch?v=dQw4w9WgXcQ".bright_blue()),"
        Commands::Placeholder => println!("Placeholder"),
        Commands::Info => gather(),
        Commands::Grep { search } => grep(search.to_owned()), //replace to_owned() with clone() if you want to use the same string again later
        Commands::Cat { femboy }=> cat(femboy.to_owned()),
    }
}
