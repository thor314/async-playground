// ping example from actix

// pub(crate) fn main_handle() {
//   // actix handler
//   let system = actix::System::new();
//   system.run();
// }
use actix::prelude::*;

// We're going to define an Actor and a Message (Ping), and define Actor as a Handler for Ping.
// This can be made even more concise with the provided declarative macros.

/// Define `Ping` message
struct Ping(usize);

impl Message for Ping {
  type Result = usize;
}

/// Actor
struct MyActor {
  count: usize,
}

/// Declare actor and its context
impl Actor for MyActor {
  type Context = Context<Self>;
}

/// Handler for `Ping` message
impl Handler<Ping> for MyActor {
  type Result = usize;

  fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
    self.count += msg.0;
    self.count
  }
}

// #[actix::main]
async fn __main() {
  let system = actix::System::new();
  let _ = system.run();
  // start new actor

  // takes the actor, turns it into a future. I guess addr is a wrapper that means handle.
  let addr = MyActor { count: 10 }.start(); // start spawns a task to manage the actor

  // Talk to my Actor. Send message and get future for result.
  let res = addr.send(Ping(10)).await;

  // handle() returns tokio handle
  println!("RESULT: {}", res.unwrap() == 20);

  // stop system and exit
  System::current().stop();
}
