use regex::Regex;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let has_not_arguments = args.len() < 2;

    if has_not_arguments {
        eprintln!("Usage: {} <path_to_svg_file>", args[0]);
        process::exit(1)
    }

    let svg_file = resolve_path( &args[1])?;

    svg_transform(&svg_file).expect("");

    Ok(())
}

fn svg_transform(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let sx_path = file_path;

    let re = Regex::new(r"(\w+)-(\w)")?;

    let mut file = File::open(sx_path).expect("Erreur: Impossible de trouver le fichier");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Erreur: Impossible de lire le fichier");

    let result = re.replace_all(&file_content, |caps: &regex::Captures| {
        format!("{}{}", &caps[1], caps[2].to_uppercase())
    });

    fs::write(sx_path, result.as_ref()).expect("Ecriture du resultat impossible");

    println!("✅ Votre fichier à été correctement convertis");

    Ok(())
}

fn resolve_path(input_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = Path::new(input_path);

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(path))
    }
}
