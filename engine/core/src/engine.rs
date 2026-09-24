use std::path::PathBuf;
use crate::{error::EngineError,fs::StorageLayout,LaunchEvent,LaunchState};

#[derive(Debug,Clone)]
pub struct LauncherEngine { pub storage:StorageLayout }

impl LauncherEngine {
 pub fn new(root:impl Into<PathBuf>)->Result<Self,EngineError>{
  let storage=StorageLayout::new(root);
  storage.ensure_dirs().map_err(|e|EngineError::RuntimeUnavailable(format!("initialize storage: {e}")))?;
  Ok(Self{storage})
 }
 pub fn event(state:LaunchState,message:impl Into<String>)->LaunchEvent{LaunchEvent::new(state,message)}
 pub fn prepare(&self)->Result<LaunchEvent,EngineError>{
  self.storage.ensure_dirs().map_err(|e|EngineError::RuntimeUnavailable(format!("prepare storage: {e}")))?;
  Ok(Self::event(LaunchState::Preparing,"engine storage is ready"))
 }
}
