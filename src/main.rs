pub mod app;
pub mod cpu;
pub mod devices;
pub mod graphics;
pub mod memory;
pub mod screen;

use app::config::Config;
use app::core::Application;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CmdArgs {
    #[arg(short, long)]
    rom_path: String,

    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let config = Config::new(String::from("config/config.yml"));
    let args = CmdArgs::parse();

    let mut app = Application::new(config);
    app.init();
    app.run();
}
