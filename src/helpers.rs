use owo_colors::colors::*;
use owo_colors::{OwoColorize, Style};
pub fn rundebug(debug: bool, mut action: impl FnMut(), mut elseaction: impl FnMut()) {
    if debug {
        println!("{}", "DEBUG BEGIN".style(debugstyle()));
        action();
        println!("{}", "DEBUG END".style(debugstyle()));
    } else {
        elseaction();
    }
}

fn debugstyle() -> Style {
    Style::new().red().on_yellow().bold().underline()
}
