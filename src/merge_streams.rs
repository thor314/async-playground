use std::future::ready;

use futures_lite::{future::block_on, prelude::*};
use merge_streams::MergeStreams;
// use merge_streams::{IntoStream, MergeStreams};

fn main() {
  block_on(async {
    let a = futures_lite::stream::once(1);
    let b = futures_lite::stream::once(2);
    let c = futures_lite::stream::once(3);
    let joinall = futures_lite::stream::iter([a, b, c].into_iter());

    let a = futures::stream::once(ready(1));
    let b = futures::stream::once(ready(2));
    let c = futures::stream::once(ready(3));
    let joinall = futures::stream::select_all([a, b, c].into_iter());

    let a = futures_lite::stream::once(1);
    let b = futures_lite::stream::once(2);
    let c = futures_lite::stream::once(3);
    let mut s = merge_streams::MergeStreams::merge(vec![a, b, c]);

    // all these types are opaque, I'd like them to suck less so I can shove them in a struct

    // let a = futures_lite::stream::once(1);
    // let b = futures_lite::stream::once(2);
    // let c = futures_lite::stream::once(3);
    // let mut s: MyStream = vec![a, b, c].merge().into_stream();
  })
}

struct MyStream;
impl Stream for MyStream {
  type Item = i32;

  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Self::Item>> {
    todo!()
  }
}
