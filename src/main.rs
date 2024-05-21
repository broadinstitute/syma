use syma::shell::run_shell;

fn main() {
    match run_shell() {
        Ok(_) => (),
        Err(error) => eprintln!("{}", error)
    }

}
