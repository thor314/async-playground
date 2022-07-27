//! A batteries-included binary template.

// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use futures::{
  channel::mpsc,
  executor::{self, ThreadPool},
  StreamExt,
};
use tokio::{spawn, task::spawn_blocking, time::sleep};
use utils::MyError;
use validator::{Validate, ValidationError};

mod actix_actor_play;
mod actor_play;
#[cfg(test)] mod tests;
mod utils;
#[tokio::main]
async fn main() -> Result<()> {
  use tokio::sync::mpsc;
  use tokio_play::*;

  let (tx, mut rx) = mpsc::channel(2);
  let start = 5;

  // use spawn blocking for synchronous apis!
  // let worker = spawn_blocking(move || {
  //   for x in 0..10 {
  //     // Stand in for complex computation
  //     println!("send {x}");
  //     tx.blocking_send(start + x).unwrap();
  //   }
  // });

  // spawn for non-blocking async:
  let worker = spawn(async move {
    for x in 0..10 {
      println!("send {x}");
      tx.send(start + x).await.unwrap();
      // tx.send(start + x); // if we don't await here, acc will be 0 when we check it.
    }
  });

  let mut acc = 0;
  while let Some(v) = rx.recv().await {
    println!("got {}", v - 5);
    acc += v;
  }
  assert_eq!(acc, 95); // if I switch blocking to non-blocking spawn, acc will be 0 when this line is reached!
                       // worker.await.unwrap(); // this isn't actually necessary

  let context = utils::setup()?;
  task_play().await;

  // but this will definitely run
  tokio::spawn(async {
    println!("I ran");
    // sleep(Duration::from_millis(1000)); // I won't actually wait, since not awaited
    sleep(Duration::from_millis(1000)).await; // I will wait
    println!("I ran, yessiree");
  })
  .await
  .unwrap();
  // this might run, but isn't awaited, since tokio tasks are non-blocking.
  tokio::spawn(async {
    println!("I might run, mebe");
    sleep(Duration::from_millis(1000)).await // note that tokio's sleep is async
  });

  Ok(())
}
// Tokio core concepts:
// - tasks - default-non-blocking units of execution. These can be made to block, with eg
//   `spawn_blocking`.
// -
//

mod tokio_play {
  use cute::c;
  use futures::future::join_all;
  // utils for tracking time. Adds features to std::time and re-exports Duration, and
  // extends/wraps some std types.
  use tokio::{
    join, spawn,
    time::{sleep, Duration},
  };
  use tokio_stream::{self as stream, StreamExt};

  async fn hi(mom: String) {
    sleep(Duration::from_secs(1)).await;
    println!("hi: {mom}");
  }

  // use tokio;
  // Tokio has 4 ma
  pub async fn task_play() {
    // this will spawn a non-blocking task that waits for a bit. Note that tokio re-exports
    // std::time::Duration.
    spawn(hi("mom1".into())).await.unwrap(); // sequential awaits
    spawn(hi("mom2".into())).await.unwrap();
    // simultaneous execution
    let handles = c!(spawn(hi(format!("mom{s}"))), for s in 3..=6);
    let _out = join_all(handles.into_iter()).await;
    // can also try to use JoinSet (requires special .cargo configuration). PRobably don't actually
    // want this.
    let mut handle_set = tokio::task::JoinSet::new();
    // can't do this because for_each expects a synchronous closure.
    // (11..=17).for_each(async |i| { handle_set.spawn(hi(format!("mom{i}"))) });
    // use async streams instead of iterators:
    for i in 10..15 {
      // these will execute simultaneously
      handle_set.spawn(async move { hi(format!("{i}")) });
    }
    while let Some(res) = handle_set.join_next().await {
      println!("test1");
    }
    // sequential awaits
    let handles = c!(spawn(hi(format!("mom{s}"))).await.unwrap(), for s in 7..=8);
  }
}

mod std_async_play {
  use std::{
    thread::{sleep, spawn},
    time::Duration,
  };

  use cute::c;
  use futures::future::join_all;

  fn hi(mom: String) {
    sleep(Duration::from_secs(1));
    println!("hi: {mom}");
  }

  pub async fn task_play() {
    spawn(|| hi("mom1".into()));
    spawn(|| hi("mom2".into()));
    let handles = c!(spawn(move || { hi(format!("mom{s}"))}), for s in 3..=6);
    for i in handles {
      i.join().unwrap();
    }
  }
}
