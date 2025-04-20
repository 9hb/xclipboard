use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{ Command, Stdio };
use dirs::home_dir;

const CLIPBOARD: &str = ".xc_clipboard";

fn ziskej_cestu_schranky() -> PathBuf {
    let mut path = home_dir().expect("xc cannot find home directory");
    path.push(CLIPBOARD);
    path
}

fn kopiruj_z_argumentu(prikaz: &str) {
    let cesta = ziskej_cestu_schranky();
    fs::write(cesta, prikaz).expect("xc cannot write to clipboard file");

    println!("📋 Copied: {}", prikaz);
}

fn vloz() {
    let cesta = ziskej_cestu_schranky();
    let prikaz = fs::read_to_string(cesta).expect("xc cannot read clipboard file");

    println!("▶️ {}", prikaz);

    let mut casti = prikaz.trim().split_whitespace();
    let spustitelny = casti.next().expect("xc cannot find command to copy");
    let argumenty: Vec<&str> = casti.collect();

    let status = Command::new(spustitelny)
        .args(argumenty)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("xc cannot execute command");

    std::process::exit(status.code().unwrap_or(1));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // zadne argumenty -> zobrazime napovedu
    if args.len() < 2 {
        eprintln!(
            "Usage: xc [arg] <command>\n   xc <command>     copy command to clipboard\n   xc -p            paste and run command from clipboard\n"
        );
        return;
    }

    match args[1].as_str() {
        "-p" => vloz(),
        _ => {
            let prikaz = if args.len() > 2 { args[1..].join(" ") } else { args[1].clone() };
            kopiruj_z_argumentu(&prikaz);
        }
    }
}
