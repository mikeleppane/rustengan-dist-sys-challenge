use std::io::{StdoutLock, Write};

use color_eyre::eyre::Context;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use dist_sys_challenge::{main_loop, Event, Init, Message, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

struct EchoNode {
    id: usize,
}

impl Node<(), Payload> for EchoNode {
    fn extract_init(&mut self, _input: Message<Payload>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_init(
        _state: (),
        _init: Init,
        _tx: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> Result<Self> {
        Ok(EchoNode { id: 1 })
    }

    fn step(&mut self, input: Event<Payload>, output: &mut StdoutLock) -> Result<()> {
        let Event::Message(input) = input else {
            panic!("got injected event when there's no event injection")
        };
        let mut reply = input.into_reply(Some(&mut self.id));

        match reply.body.payload {
            Payload::Echo { echo } => {
                reply.body.payload = Payload::EchoOk { echo };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to ")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    main_loop::<_, EchoNode, _, _>(())
}
