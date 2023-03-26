fn main() {
    if let Err(e) = nix2dos::get_args().and_then(nix2dos::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}