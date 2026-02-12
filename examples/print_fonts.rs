fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-fonts-dir>", args[0]);
        std::process::exit(1);
    }

    let path = std::path::Path::new(&args[1]);
    if !path.exists() {
        eprintln!("Error: Directory '{}' does not exist.", path.display());
        std::process::exit(1);
    }

    let mut db = fontdb::Database::new();
    let now = std::time::Instant::now();
    db.load_fonts_dir(path);

    println!(
        "Loaded {} font faces in {}ms.",
        db.len(),
        now.elapsed().as_millis()
    );

    for face in db.faces() {
        let family = face
            .families
            .first()
            .map(|(name, _)| name.as_str())
            .unwrap_or("Unknown");
        let path_display = match &face.source {
            fontdb::Source::File(p) => p.display().to_string(),
            fontdb::Source::SharedFile(p, _) => p.display().to_string(),
            fontdb::Source::Binary(_) => "Binary data".to_string(),
        };

        println!(
            "Family: {}, Style: {:?}, Weight: {:?}, Stretch: {:?}, Path: {}",
            family, face.style, face.weight, face.stretch, path_display
        );
    }
}
