use cute::c;
use futures::future::join_all;
// utils for tracking time. Adds features to std::time and re-exports Duration, and
// extends/wraps some std types.
use tokio::{
  join, spawn,
  time::{sleep, Duration},
};
use tracing::{info, instrument};

#[instrument]
async fn hi(mom: String) {
  sleep(Duration::from_secs(1)).await;
  info!("hi: {mom}");
}

// tokio: use to spawn async threads. This function demonstrates simultaneous and sequential
// execution.
//
// Futures will not start by themselves. The tokio spawn api drives them to start, and the
// join.await api ensures that they finish.
pub async fn task_play() {
  // this will spawn a non-blocking task that waits for a bit.
  // Note that tokio re-exports std::time::Duration.

  // PARALLEL execution via the spawn api
  let handles = c!(spawn(hi(format!("mom{s}"))), for s in 0..=5);
  let _out = join_all(handles.into_iter()).await;
  println!("\n");

  // basically identical, using the joinSet API and incrementally, join_next
  let mut handle_set = tokio::task::JoinSet::new();
  for i in 10..15 {
    handle_set.spawn(async move { hi(format!("{i}")).await });
  }
  println!("\n");
  while let Some(res) = handle_set.join_next().await {
    tracing::warn!("test {res:?}");
  }
  println!("\n");

  // async closures are unstable unfortunately
  // let a = spawn(async || info!("s") ); // no
  // let a = spawn(hi("s".into())); // this is fine though.

  // most concisely (though maybe not the easiest to read):
  let joined = join_all(c!(spawn(hi(format!("{i}"))), for i in 20..=25)).await;
  // equivalent without cute:
  let joined = join_all((25..=30).map(|i| spawn(hi(format!("{i}"))))).await;

  // without the spawn api, these futures will never begin, until they are awaited
  let slow_start = join_all((30..=35).map(|i| hi(format!("{i}"))));

  // let joined = join_all(joined).await;
  // let joined = join!(c!(spawn(hi("{i}".into())), for i in 0..=5));

  // SEQUENTIAL
  spawn(hi("mom1".into())).await.unwrap(); // sequential awaits
  spawn(hi("mom2".into())).await.unwrap();
  println!("\n");
  let _handles = c!(spawn(hi(format!("mom{s}"))).await.unwrap(), for s in 3..=8);
  slow_start.await;
}
