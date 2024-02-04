#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;

mod actor;
mod channels;
mod error;
mod mutex;
mod pin_project;
mod my_future;
mod stream;
mod tasks;
mod executor;
#[cfg(test)] mod tests;
mod utils;

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), MyError> {
  utils::setup()?;
  info!("hello thor");
  // tokio_play::task_play().await;
  stream::streams().await;

  Ok(())
}
