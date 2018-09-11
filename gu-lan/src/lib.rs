#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate gu_actix;
extern crate gu_p2p;
extern crate gu_base;

extern crate serde_json;
extern crate bytes;
extern crate socket2;

extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate dns_parser;
extern crate futures;
extern crate tokio;
extern crate tokio_codec;

mod errors;
mod mdns_codec;
pub mod rest_client;
pub mod server;
pub mod resolve_actor;
pub mod service;

pub const LAN_ENDPOINT : u32 = 576411;