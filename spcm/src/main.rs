extern crate clap;
extern crate simplelog;
extern crate chrono;

use clap as cl;
use simplelog as slog;
use std::fs;
use chrono::offset::Local as time;
use std::env;
//use std::io::ErrorKind;

const PACKAGENAME: &str = "spcm";
const PACKAGEFULLNAME: &str = "simplepacman";


fn main() {
	let logfiledir: String = env::var("HOME").unwrap().to_string() + &"/.local/share/" + &PACKAGEFULLNAME.to_string() + &"/logs/".to_string();
	let logfilepath: String = logfiledir.to_string() + &time::now().format("%Y-%m-%d_%H-%M-%S-%3f").to_string() + &".log".to_string();

	// Parsing Options
	let m = cl::App::new("Simple Pacman Wrapper")
		.version("0.1.0")
		.author("LeSnake04 <dev.lesnake@posteo.de>")
		.about("Simple, yet Powerful pacman Wrapper")
		.setting(cl::AppSettings::ArgRequiredElseHelp)
		.arg(cl::Arg::with_name("install")
			.long("install")
			.short("i")
			.value_name("PKGS")
		)
	.get_matches();

	println!("Dir: {}, Path: {}",&logfiledir , &logfilepath);
	fs::create_dir_all(&logfiledir).unwrap();
	slog::CombinedLogger::init(vec![
		slog::TermLogger::new(slog::LevelFilter::Debug, slog::Config::default(), slog::TerminalMode::Mixed, slog::ColorChoice::Auto),
		slog::WriteLogger::new(slog::LevelFilter::Debug, slog::Config::default(), fs::File::create(&logfilepath).unwrap()),
	]).unwrap();

	println!("install: {}", m.value_of("install").unwrap_or("Error unwrapping"));
}
