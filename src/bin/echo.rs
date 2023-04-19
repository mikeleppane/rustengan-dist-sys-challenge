use std::io::{StdoutLock, Write};

use color_eyre::eyre::Context;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use dist_sys_challenge::{main_loop, Body, Init, Message, Node};

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
    fn extract_init(&mut self, input: Message<Payload>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_init(_state: (), _init: Init) -> Result<Self> {
        Ok(EchoNode { id: 1 })
    }

    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
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
    main_loop::<_, EchoNode, _>(())
}
