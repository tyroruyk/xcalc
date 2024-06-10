mod app;
mod input;
mod mode;
mod ui;
mod utils;

fn main() -> std::io::Result<()> {
    app::run_app()
}
