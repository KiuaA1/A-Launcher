use serde::{Deserialize,Serialize};
use std::time::{SystemTime,UNIX_EPOCH};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
pub enum LaunchState { Preparing, Resolving, Downloading, Starting, Running, Exited, Failed, Cancelled }

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LaunchEvent {
 pub state:LaunchState,
 pub message:String,
 pub timestamp_ms:u64,
 pub exit_code:Option<i32>,
}

impl LaunchEvent {
 pub fn new(state:LaunchState,message:impl Into<String>)->Self {
  Self{state,message:message.into(),timestamp_ms:SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64,exit_code:None}
 }
 pub fn exited(code:i32)->Self { Self{state:LaunchState::Exited,message:format!("Minecraft exited with code {code}"),timestamp_ms:SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64,exit_code:Some(code)} }
}

pub mod artifacts;
pub mod arguments;
pub mod cache;
pub mod classpath;
pub mod download;
pub mod error;
pub mod fs;
pub mod instance;
pub mod launch;
pub mod manifest;
pub mod mojang;
pub mod native;
pub mod plan;
pub mod process;
pub mod resolver;
pub mod runtime;

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn creates_event(){let e=LaunchEvent::new(LaunchState::Preparing,"test");assert_eq!(e.state,LaunchState::Preparing);assert!(!e.message.is_empty());}
}
