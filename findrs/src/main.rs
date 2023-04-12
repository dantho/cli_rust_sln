fn main() {
    if let Err(e) = findrs::get_args().and_then(findrs::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
