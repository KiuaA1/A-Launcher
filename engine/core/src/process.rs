use crate::{error::EngineError,launch::LaunchPlan};
use std::{process::{Child,Command,Stdio},sync::{Arc,Mutex}};

pub trait ProcessManager {
 fn validate_launch(&self,plan:&LaunchPlan)->Result<(),EngineError>;
 fn spawn(&self,plan:&LaunchPlan)->Result<ManagedProcess,EngineError>;
}

pub struct DefaultProcessManager;

pub struct ManagedProcess { child:Arc<Mutex<Option<Child>>> }

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ProcessState { Running,Exited(i32),Signaled,Unavailable }

impl ManagedProcess {
 pub fn try_state(&self)->Result<ProcessState,EngineError>{
  let mut guard=self.child.lock().map_err(|_|EngineError::RuntimeUnavailable("process lock poisoned".into()))?;
  let child=guard.as_mut().ok_or_else(||EngineError::RuntimeUnavailable("process handle unavailable".into()))?;
  match child.try_wait().map_err(|e|EngineError::RuntimeUnavailable(format!("check process state: {e}")))? {
   Some(status)=>Ok(ProcessState::Exited(status.code().unwrap_or(-1))),
   None=>Ok(ProcessState::Running),
  }
 }
 pub fn wait(&self)->Result<ProcessState,EngineError>{
  let mut guard=self.child.lock().map_err(|_|EngineError::RuntimeUnavailable("process lock poisoned".into()))?;
  let child=guard.as_mut().ok_or_else(||EngineError::RuntimeUnavailable("process handle unavailable".into()))?;
  let status=child.wait().map_err(|e|EngineError::RuntimeUnavailable(format!("wait for process: {e}")))?;
  Ok(ProcessState::Exited(status.code().unwrap_or(-1)))
 }
 pub fn kill(&self)->Result<(),EngineError>{
  let mut guard=self.child.lock().map_err(|_|EngineError::RuntimeUnavailable("process lock poisoned".into()))?;
  let child=guard.as_mut().ok_or_else(||EngineError::RuntimeUnavailable("process handle unavailable".into()))?;
  child.kill().map_err(|e|EngineError::RuntimeUnavailable(format!("kill process: {e}")))
 }
}

impl ProcessManager for DefaultProcessManager {
 fn validate_launch(&self,plan:&LaunchPlan)->Result<(),EngineError>{plan.validate()}
 fn spawn(&self,plan:&LaunchPlan)->Result<ManagedProcess,EngineError>{
  self.validate_launch(plan)?;
  let mut command=Command::new(&plan.java_executable);
  command.args(&plan.jvm_args);
  command.args(&plan.game_args);
  command.current_dir(&plan.game_directory);
  command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
  let child=command.spawn().map_err(|e|EngineError::RuntimeUnavailable(format!("spawn Java process: {e}")))?;
  Ok(ManagedProcess{child:Arc::new(Mutex::new(Some(child)))})
 }
}
