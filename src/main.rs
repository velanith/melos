mod config;
mod dsp;

fn main() -> anyhow::Result<()> {
    let config = config::config::Config::get();
    println!("{:?}", config.input.input_path);
    Ok(())
}
