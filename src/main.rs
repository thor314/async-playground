#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;

mod error;
#[cfg(test)] mod tests;
mod utils;
mod tokio_play;
mod actor;
mod stream;

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), MyError> {
  utils::setup()?;
  info!("hello thor");
  // tokio_play::task_play().await;
  stream::streams().await;

  Ok(())
}
