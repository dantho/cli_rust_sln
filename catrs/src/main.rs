fn main() {
    if let Err(e) = catrs::get_args().and_then(catrs::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}