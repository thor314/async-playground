use std::future::ready;

use futures::{
  executor::block_on,
  future::{self, join_all},
  stream::{self, once, BoxStream},
  StreamExt,
};

// the Rust stream APIs suck
pub async fn streams() {
  block_on(async {
    // stream some values:
    // once(future)
    let a = stream::once(async { 0 });
    let b = stream::once(async { 1 });
    let c = stream::once(async { 2 });
    // let joinall = stream::iter([a, b, c].into_iter().flatten());
    let x = future::ready(a);
    let y = future::ready(b);
    let z = future::ready(c);
    // let joinall = stream::iter([a, b, c].into_iter().flatten());
    let it = stream::iter(1..3);

    let a = stream::once(ready(1));
    let b = stream::once(ready(1));
    let c = stream::once(ready(2));
    // Use select_all to combine the streams
    let combined_stream = stream::select_all(vec![a, b, c]);

    // let joinall = stream::select_all([a,b].into_iter()); // bundle several streams into one
    // let stream: BoxStream<'static, i32> = Box::pin(joinall);

    // let v = vec![x, y, z];
    // let vv = futures::stream::iter([x, y, z]);
    // // let vv = tokio_stream::iter([x,y,z]); // equiv
    // let responses = join_all(v);

    // // let s = merge_streams::MergeStreams::merge(vec![a, b, c]);

    // // all these types are opaque, I'd like them to suck less so I can shove them in a struct
    // let stream: BoxStream<'static, i32> = Box::pin(s);

    // let a = futures::stream::once(1);
    // let b = futures::stream::once(2);
    // let c = futures::stream::once(3);
    // let mut s: MyStream = vec![a, b, c].merge().into_stream();
  })
}
