use serde::{Deserialize,Serialize};
use std::time::{SystemTime,UNIX_EPOCH};
use crate::{error::EngineError,process::{ManagedProcess,ProcessEvent}};

fn now_ms()->u64{SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
pub enum SessionStatus{Preparing,Starting,Running,Exited,Failed,Cancelled}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LaunchSession{
 pub id:String,
 pub minecraft_version:String,
 pub status:SessionStatus,
 pub started_at_ms:u64,
 pub ended_at_ms:Option<u64>,
 pub exit_code:Option<i32>,
 pub logs:Vec<String>,
}

impl LaunchSession{
 pub fn new(id:impl Into<String>,version:impl Into<String>)->Self{Self{id:id.into(),minecraft_version:version.into(),status:SessionStatus::Preparing,started_at_ms:now_ms(),ended_at_ms:None,exit_code:None,logs:Vec::new()}}
 pub fn apply(&mut self,event:ProcessEvent){
  match event{
   ProcessEvent::Started=>self.status=SessionStatus::Starting,
   ProcessEvent::Stdout(line)=>{self.status=SessionStatus::Running;self.logs.push(line);},
   ProcessEvent::Stderr(line)=>{self.status=SessionStatus::Running;self.logs.push(line);},
   ProcessEvent::Exited(code)=>{self.exit_code=Some(code);self.status=if code==0{SessionStatus::Exited}else{SessionStatus::Failed};self.ended_at_ms=Some(now_ms());}
  }
 }
 pub fn cancel(&mut self){self.status=SessionStatus::Cancelled;self.ended_at_ms=Some(now_ms());}
}

#[derive(Debug,Default)]
pub struct LaunchSessionRegistry{sessions:std::collections::HashMap<String,LaunchSession>}

impl LaunchSessionRegistry{
 pub fn insert(&mut self,session:LaunchSession){self.sessions.insert(session.id.clone(),session);}
 pub fn get(&self,id:&str)->Option<&LaunchSession>{self.sessions.get(id)}
 pub fn get_mut(&mut self,id:&str)->Option<&mut LaunchSession>{self.sessions.get_mut(id)}
 pub fn remove(&mut self,id:&str)->Option<LaunchSession>{self.sessions.remove(id)}
 pub fn apply_process_events(&mut self,id:&str,process:&ManagedProcess)->Result<usize,EngineError>{
  let session=self.sessions.get_mut(id).ok_or_else(||EngineError::RuntimeUnavailable(format!("launch session not found: {id}")))?;
  let mut count=0;
  for event in process.events().try_iter(){session.apply(event);count+=1;}
  Ok(count)
 }
}
