use crate::{error::EngineError,launch::LaunchPlan};
use std::{collections::VecDeque,io::{BufRead,BufReader},process::{Child,Command,Stdio},sync::{Arc,Mutex}};

pub trait ProcessManager {
 fn validate_launch(&self,plan:&LaunchPlan)->Result<(),EngineError>;
 fn spawn(&self,plan:&LaunchPlan)->Result<ManagedProcess,EngineError>;
}
pub struct DefaultProcessManager;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ProcessEvent { Started, Stdout(String), Stderr(String), Exited(i32) }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ProcessState { Running,Exited(i32),Signaled,Unavailable }

struct ProcessInner {
 child:Option<Child>,
 events:VecDeque<ProcessEvent>,
 started:bool,
 exited:bool,
}
pub struct ManagedProcess { inner:Arc<Mutex<ProcessInner>> }

impl ManagedProcess {
 pub fn drain_events(&self)->Vec<ProcessEvent>{
  let mut inner=match self.inner.lock(){Ok(v)=>v,Err(_)=>return Vec::new()};
  inner.events.drain(..).collect()
 }
 pub fn try_state(&self)->Result<ProcessState,EngineError>{
  let mut inner=self.inner.lock().map_err(|_|EngineError::RuntimeUnavailable("process lock poisoned".into()))?;
  let child=inner.child.as_mut().ok_or_else(||EngineError::RuntimeUnavailable("process handle unavailable".into()))?;
  match child.try_wait().map_err(|e|EngineError::RuntimeUnavailable(format!("check process state: {e}")))? {
   Some(status)=>{let code=status.code().unwrap_or(-1);if !inner.exited{inner.events.push_back(ProcessEvent::Exited(code));inner.exited=true;}Ok(ProcessState::Exited(code))},
   None=>Ok(ProcessState::Running)
  }
 }
 pub fn wait(&self)->Result<ProcessState,EngineError>{
  let mut inner=self.inner.lock().map_err(|_|EngineError::RuntimeUnavailable("process lock poisoned".into()))?;
  let child=inner.child.as_mut().ok_or_else(||EngineError::RuntimeUnavailable("process handle unavailable".into()))?;
  let status=child.wait().map_err(|e|EngineError::RuntimeUnavailable(format!("wait for process: {e}")))?;
  let code=status.code().unwrap_or(-1);
  if !inner.exited{inner.events.push_back(ProcessEvent::Exited(code));inner.exited=true;}
  Ok(ProcessState::Exited(code))
 }
 pub fn kill(&self)->Result<(),EngineError>{
  let mut inner=self.inner.lock().map_err(|_|EngineError::RuntimeUnavailable("process lock poisoned".into()))?;
  let child=inner.child.as_mut().ok_or_else(||EngineError::RuntimeUnavailable("process handle unavailable".into()))?;
  child.kill().map_err(|e|EngineError::RuntimeUnavailable(format!("kill process: {e}")))
 }
}

fn start_event_pump(inner:Arc<Mutex<ProcessInner>>,stdout:Option<std::process::ChildStdout>,stderr:Option<std::process::ChildStderr>){
 let waiter_inner=inner.clone();
 std::thread::spawn(move||{
  let status=loop {
   let result=match waiter_inner.lock(){
    Ok(mut i)=>{
     match i.child.as_mut(){
      Some(child)=>child.try_wait(),
      None=>return,
     }
    },
    Err(_)=>return,
   };
   match result {
    Ok(Some(status))=>break status,
    Ok(None)=>std::thread::sleep(std::time::Duration::from_millis(50)),
    Err(_)=>return,
   }
  };
  if let Ok(mut i)=waiter_inner.lock(){
   if !i.exited {
    let code=status.code().unwrap_or(-1);
    i.events.push_back(ProcessEvent::Exited(code));
    i.exited=true;
   }
  }
 });

 std::thread::spawn(move||{
  let mut handles=Vec::new();
  if let Some(out)=stdout{let shared=inner.clone();handles.push(std::thread::spawn(move||{for line in BufReader::new(out).lines().flatten(){if let Ok(mut i)=shared.lock(){i.events.push_back(ProcessEvent::Stdout(line));}}}));}
  if let Some(err)=stderr{let shared=inner.clone();handles.push(std::thread::spawn(move||{for line in BufReader::new(err).lines().flatten(){if let Ok(mut i)=shared.lock(){i.events.push_back(ProcessEvent::Stderr(line));}}}));}
  for h in handles{let _=h.join();}
 });
}

impl ProcessManager for DefaultProcessManager {
 fn validate_launch(&self,plan:&LaunchPlan)->Result<(),EngineError>{plan.validate()}
 fn spawn(&self,plan:&LaunchPlan)->Result<ManagedProcess,EngineError>{
  self.validate_launch(plan)?;
  let mut command=Command::new(&plan.java_executable);
  command.args(&plan.jvm_args).args(&plan.game_args).current_dir(&plan.game_directory).envs(&plan.environment)
   .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
  let mut child=command.spawn().map_err(|e|EngineError::RuntimeUnavailable(format!("spawn Java process: {e}")))?;
  let stdout=child.stdout.take();let stderr=child.stderr.take();
  let inner=Arc::new(Mutex::new(ProcessInner{child:Some(child),events:VecDeque::from([ProcessEvent::Started]),started:true,exited:false}));
  start_event_pump(inner.clone(),stdout,stderr);
  Ok(ManagedProcess{inner})
 }
}
