const NAME: &str = "SyMa";
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const DESCRIPTION: Option<&str> = option_env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
const HOMEPAGE: Option<&str> = option_env!("CARGO_PKG_HOMEPAGE");
const REPOSITORY: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

fn non_empty(s: Option<&str>) -> Option<&str> {
    match s {
        Some("") => None,
        s => s,
    }
}
pub fn print_intro() {
    match non_empty(VERSION) {
        None => { println!("Welcome to {NAME}!"); }
        Some(version) => { println!("Welcome to {NAME} Version {version}!"); }
    }
    match (non_empty(DESCRIPTION), non_empty(AUTHORS)) {
        (Some(description), Some(authors)) => {
            println!("{description} by {authors}");
        }
        (Some(description), None) => {
            println!("{description}");
        }
        (None, Some(authors)) => {
            println!("Written by {authors}");
        }
        (None, None) => {}
    }
    let link = non_empty(HOMEPAGE).or(non_empty(REPOSITORY));
    if let Some(link) = link {
        println!("For more information and latest version, check {link}");
    }
}