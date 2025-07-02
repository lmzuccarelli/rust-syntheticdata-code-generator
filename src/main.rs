use clap::Parser;
use cli::schema::*;
use custom::error::GenerateError;
use custom_logger as log;
use data::generate::*;
use std::fs;
use std::str::FromStr;

mod cli;
mod custom;
mod data;

fn main() -> Result<(), GenerateError> {
    let args = Cli::parse();
    let level = args.loglevel.as_deref().unwrap_or("info");
    let config = args.config;

    let res_log_level = log::LevelFilter::from_str(level)
        .map_err(|_| GenerateError::new(&format!("invalid log level \"{level}\"")))?;

    // setup logging
    log::Logging::new()
        .with_level(res_log_level)
        .init()
        .expect("should initialize");

    let mode = args.mode;
    match mode {
        Mode::Generate => {
            let impl_gen = ImplGeneratorInterface {};
            let params = impl_gen.read(config.to_string());
            let _ = impl_gen.create(params.unwrap());
            log::info!("completed code generation");
        }
        Mode::Execute => {
            let records = match args.count {
                Some(v) => v,
                None => 10,
            };
            let name = match args.name {
                Some(v) => v,
                None => "no-name".to_string(),
            };

            fs::create_dir_all("results").expect("should create results directory");
            // change this section below to include the specific library for generation
            #[cfg(feature = "use-library")]
            let mut sd = sd_gen_queuemetrics::QueueMetrics::new(name.clone(), records);
            #[cfg(feature = "use-library")]
            sd.generate();
            log::info!("artifacts created in results folder");
            log::info!("synthetic code created for {} with count {}", name, records);
        }
    }
    Ok(())
}
