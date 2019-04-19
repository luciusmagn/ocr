/// detekuje číslo na obrázku, viz zadání
/// před kompilací je potřeba nainstalovat tesseract tak,
/// aby libleptonica a libtesseract byly v PATH
/// a nainstalovat ces language pack pro tesseract
///
/// ### Kompilace:
/// ```bash
/// cargo build --release
/// ```
/// Binární soubor bude target/release/odmociny
///
/// ### Spouštení přes Cargo
/// ```bash
/// cargo run --release
/// ```
///
/// moje prostředí: Arch Linux x86_64, GCC, Rust 1.34.0-nightly (c1d2d83ca 2019-03-01)
extern crate tesseract;
extern crate promptly;
extern crate yansi;

use yansi::Paint;
use promptly::prompt;

use std::fs::read_dir;
use std::process::exit;

fn main() {
    let cesta: String = prompt(format!("{}", Paint::yellow("Zadejte cestu ke složce")));

    eprintln!("{}", Paint::yellow("Vyhodnocuji.."));

    // projde složku a všechno prožene tesseractem
    match read_dir(&cesta) {
    	Err(e) => {
			eprintln!("Nepodařilo se přečíst složku: {}", e);
			exit(-1)
    	},
    	// filtrování
    	Ok(r) => for soubor in r.filter(|e| e.is_ok())
    		.map(|e| e.unwrap())
    		.map(|f| f.path())
    		.filter(|f| !f.is_dir())
    		.map(|f| f.to_str()
    			.unwrap()
    			.to_string())
    		.filter(|s| s.to_lowercase().contains("png"))
    	{
    		let mut t = tesseract::Tesseract::new();
    		t.set_lang("ces");
    		t.set_image(&soubor);
    		t.recognize();

    		match t.get_text() {
    			a @ "1" | a @ "2" |
    			a @ "3" | a @ "4" |
    			a @ "5" | a @ "6" |
    			a @ "7" | a @ "8" |
    			a @ "9" | a @ "0"z
    				=> println!("{} - {}", soubor, a),
    			_ => println!("{} - X", soubor)
    		}
    	}
    }
}
