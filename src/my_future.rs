use std::{
  future::Future,
  pin::Pin,
  sync::{
    mpsc::{sync_channel, Receiver, SyncSender},
    Arc, Mutex,
  },
  task::{Context, Poll, Waker},
  thread,
  time::Duration,
};

use futures::{
  future::{BoxFuture, FutureExt},
  task::{waker_ref, ArcWake},
};

// now you try.
#[derive(Debug, Default)]
pub struct ThorFuture {
  shared_office: Arc<Mutex<SharedOffice>>,
}

// Thor's shared office is ready when there are two people in it
#[derive(Debug, Default)]
pub struct SharedOffice {
  occupants: u8,
  waker:     Option<Waker>,
}

impl Future for ThorFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut shared_office = self.shared_office.lock().unwrap();
    // if the office is shared, it's "ready"... for a standup I guess.
    if shared_office.occupants >= 2 {
      Poll::Ready(())
    } else {
      // otherwise, there's not enough people in it. Pending!
      shared_office.waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }
}

impl ThorFuture {
  pub fn new() -> Self {
    let shared_office = Arc::new(Mutex::new(SharedOffice { occupants: 0, waker: None }));
    let thread_shared = shared_office.clone();
    thread::spawn(move || {
      let mut shared_state = thread_shared.lock().unwrap();
      if let Some(waker) = shared_state.waker.take() {
        println!("waker: {waker:?}");
        waker.wake()
      }
    });
    Self { shared_office }
  }

  pub fn person_leave(&mut self) {
    let mut office = self.shared_office.lock().unwrap();
    assert!(office.occupants > 0);
    office.occupants -= 1;
  }

  pub fn person_enter(&mut self) {
    let mut office = self.shared_office.lock().unwrap();
    office.occupants += 1;
  }
}
