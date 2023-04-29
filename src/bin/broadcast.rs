use std::collections::HashMap;
use std::io::{StdoutLock, Write};

use color_eyre::eyre::Context;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use dist_sys_challenge::{main_loop, Init, Message, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

#[allow(dead_code)]
struct BroadcastNode {
    node: String,
    id: usize,
    messages: Vec<usize>,
}

impl Node<(), Payload> for BroadcastNode {
    fn extract_init(&mut self, _input: Message<Payload>) -> Result<Self> {
        todo!()
    }

    fn from_init(_state: (), init: Init) -> Result<Self> {
        Ok(Self {
            node: init.node_id,
            id: 1,
            messages: vec![],
        })
    }

    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> Result<()> {
        let mut reply = input.into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Broadcast { message } => {
                self.messages.push(message);
                reply.body.payload = Payload::BroadcastOk;
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to broadcast")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }
            Payload::Read => {
                reply.body.payload = Payload::ReadOk {
                    messages: self.messages.clone(),
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to read")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }
            Payload::Topology { topology: _ } => {
                reply.body.payload = Payload::TopologyOk;
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to topology")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }
            Payload::BroadcastOk | Payload::ReadOk { .. } | Payload::TopologyOk => {}
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
