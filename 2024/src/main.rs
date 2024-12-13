mod tasks;
use tracing_subscriber;

fn main() {
  // Initialize tracing subscriber to display log output
  tracing_subscriber::fmt::init();
  tasks::plutonian_pebbles::run();
}
