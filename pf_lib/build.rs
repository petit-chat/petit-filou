fn main() {
    let features = [
        cfg!(feature = "mp4"),
        cfg!(feature = "avi"),
        cfg!(feature = "flv"),
        cfg!(feature = "mov"),
        cfg!(feature = "mpeg"),
        cfg!(feature = "webm"),
        cfg!(feature = "wmv"),
    ];

    if !features.iter().any(|&enabled| enabled) {
        eprintln!("Error: At least one feature must be enabled.");
        std::process::exit(1);
    }
}
