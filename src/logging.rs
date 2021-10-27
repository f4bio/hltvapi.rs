use itconfig::get_env_or_default;
use tracing::debug;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::errors::ApiError;

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn initialize() -> Result<(), ApiError> {
  let app_name: String = get_env_or_default("APP_NAME", "hltvapi");
  debug!("app name: '{}'", app_name);

  let app_log: String = get_env_or_default("APP_LOG_LEVEL", "info");
  debug!("app log: '{}'", app_log);

  let log_level: String = format!("warn,{}={}", app_name, app_log);
  debug!("log level: '{}'", log_level);

  let formatting_layer = BunyanFormattingLayer::new(app_name, std::io::stdout);
  let subscriber = Registry::default()
    .with(EnvFilter::new(log_level))
    .with(JsonStorageLayer)
    .with(formatting_layer);

  LogTracer::init().expect("Unable to setup log tracer!");

  Ok(set_global_default(subscriber).unwrap())
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_initialize() {
    assert_eq!(initialize().unwrap(), ());
  }
}
