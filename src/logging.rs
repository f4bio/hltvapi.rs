use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber(name: String, level: String) -> impl Subscriber + Send + Sync {
  let env_filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new(format!("warn,{}={}", name, level)));
  let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
  Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
  LogTracer::init().expect("Failed to set logger");
  set_global_default(subscriber).expect("Failed to set subscriber");
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_initialize() {
    let subscriber = get_subscriber("test_module".into(), "debug".into());

    assert_eq!(init_subscriber(subscriber), ());
  }
}
