// https://ryhl.io/blog/actors-with-tokio/

// Actor (n.): a task and a handle.
// Why write it this way? With a Message that has a responder in the Message body?
// We didn't have to, but it allows our actor to respond with information.
//
// Another question, why use an ActorHandle that doesn't hold `run_actor`? Couldn't we have put
// `run_actor` under ActorHandle or Actor:
// pub async fn run(&mut self) { // in Actor
// tokio::spawn(async move {
//   while let Some(msg) = self.receiver.recv().await {
//     self.handle_message(msg);
//   }
// });
// }
// // And get rid of ActorHandle.
//
// 2 issues described:
// - tokio::spawn now has to live inside the `run` method. This means the task must own everything
//   inside of it. The method borrows self, meaning it cannot give ownernship of self to the task.
//   I'm not sure I understand this as well, but....
// - The actor and handle are the same struct. This means we can't clone the Handle for more message
//   passers. I do get this.
//
// See below for an example of how to move run_actor into the body of Actor, returning the handle.
use tokio::sync::{mpsc, oneshot};
mod alice {
  use super::*;
  // api to expose in main
  pub async fn run_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
      actor.handle_message(msg);
    }
  }

  // derive a Message type. , and a way for the message to respond "done"
  #[derive(Debug)]
  pub enum Message {
    Base { info: usize, responder: oneshot::Sender<usize> },
  }

  // derive an actor as a worker that can receive messages from some handle owner
  #[derive(Debug)]
  pub struct Actor {
    receiver:  mpsc::Receiver<Message>,
    last_info: usize,
  }

  // Finally, we need a hnadle to the actor to hold onto, keeping the actor in scope.
  // Note the Clone; since `mpsc` iplements clone, we can create further message-passers to the
  // Actor.
  #[derive(Debug, Clone)]
  pub struct ActorHandle {
    sender: mpsc::Sender<Message>,
  }

  impl Message {
    fn new() -> (oneshot::Receiver<usize>, Self) {
      let (tx, rx) = oneshot::channel();
      (rx, Self::Base { info: 0, responder: tx })
    }
  }

  impl Actor {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
      {
        Self { receiver, last_info: 0 }
      }
    }

    // got a message! do a thing, and respond to it
    fn handle_message(&mut self, msg: Message) {
      match msg {
        Message::Base { mut info, responder } => {
          std::mem::swap(&mut self.last_info, &mut info);
          let _ = responder.send(info);
        },
      }
    }
  }

  impl ActorHandle {
    pub fn new() -> Self {
      {
        let (tx, rx) = mpsc::channel(8);
        let actor = Actor::new(rx);
        Self { sender: tx }
      }
    }

    pub async fn get_info(&self) -> usize {
      let (rx, msg) = Message::new();
      rx.await.expect("Light! I neeed light! Dead Actor!")
    }
  }
}

mod no_handle {
  use super::*;
  // moved into Actor
  // pub async fn run_actor(mut actor: Actor) {
  //   while let Some(msg) = actor.receiver.recv().await {
  //     actor.handle_message(msg);
  //   }
  // }

  // derive a Message type. , and a way for the message to respond "done"
  #[derive(Debug)]
  pub enum Message {
    Base { info: usize, responder: oneshot::Sender<usize> },
  }

  // derive an actor as a worker that can receive messages from some handle owner
  #[derive(Debug)]
  pub struct Actor {
    receiver:  mpsc::Receiver<Message>,
    last_info: usize,
  }

  // different from above: manages spawn, since run moved into Actor impl body
  #[derive(Debug, Clone)]
  pub struct ActorHandle {
    sender: mpsc::Sender<Message>,
  }

  impl Message {
    fn new() -> (oneshot::Receiver<usize>, Self) {
      let (tx, rx) = oneshot::channel();
      (rx, Self::Base { info: 0, responder: tx })
    }
  }

  impl Actor {
    // no good: borrowed self escapes into the task
    // pub async fn run(&mut self) {
    //   // borrowed data escapes function: self escapes the associated function body
    //   tokio::spawn(async move {
    //     while let Some(msg) = self.receiver.recv().await {
    //       self.handle_message(msg);
    //     }
    //   });
    // }

    // have to add an actor handle to spawn the actor, see below
    async fn run(&mut self) {
      while let Some(msg) = self.receiver.recv().await {
        self.handle_message(msg);
      }
    }

    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
      {
        Self { receiver, last_info: 0 }
      }
    }

    // got a message! do a thing, and respond to it
    fn handle_message(&mut self, msg: Message) {
      match msg {
        Message::Base { mut info, responder } => {
          std::mem::swap(&mut self.last_info, &mut info);
          let _ = responder.send(info);
        },
      }
    }
  }

  impl ActorHandle {
    pub fn new() -> Self {
      {
        let (tx, rx) = mpsc::channel(8);
        let mut actor = Actor::new(rx);
        // changed: spawn a task to run the actor, and return handle to self
        tokio::spawn(async move { actor.run().await });

        Self { sender: tx }
      }
    }

    pub async fn get_info(&self) -> usize {
      let (rx, msg) = Message::new();
      rx.await.expect("Light! I neeed light! Dead Actor!")
    }
  }
}

mod actix {
  // Actix makes a lot of this implicit: MyActor and message handle receivers implicitly.
  // The handler is automatically handled. Lot of syntax sugaring away.
  use ::actix::prelude::*;

  use super::*;
  // derive an actor as a worker that can receive messages from some handle owner
  #[derive(Debug)]
  pub struct MyActor {
    // receiver:  mpsc::Receiver<MyMessage>,
    last_info: usize,
  }

  #[derive(Debug)]
  pub enum MyMessage {
    Base {
      info: usize, // , responder: oneshot::Sender<usize>
    },
  }

  impl Message for MyMessage {
    type Result = usize;
  }

  impl Actor for MyActor {
    type Context = Context<Self>;
  }

  impl Handler<MyMessage> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: MyMessage, ctx: &mut Self::Context) -> Self::Result {
      match msg {
        // MyMessage::Base { mut info, responder } => {
        MyMessage::Base { mut info } => {
          std::mem::swap(&mut self.last_info, &mut info);
          // let _ = responder.send(info);
          info
        },
      }
    }
  }

  impl MyMessage {
    // fn new() -> (oneshot::Receiver<usize>, Self) {
    fn new() -> Self {
      // let (tx, rx) = oneshot::channel();
      // (rx, Self::Base { info: 0, responder: tx })
      Self::Base { info: 0 }
    }
  }

  impl MyActor {
    pub fn new(receiver: mpsc::Receiver<MyMessage>) -> Self {
      {
        Self {
          // receiver,
          last_info: 0,
        }
      }
    }
  }
}
