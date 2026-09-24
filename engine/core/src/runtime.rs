use serde::{Deserialize, Serialize};
use std::{path::{Path,PathBuf},process::Command};
use crate::error::EngineError;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct JavaRuntime { pub id:String,pub version:String,pub executable:PathBuf }

impl JavaRuntime {
 pub fn validate(&self)->Result<(),EngineError>{
  if self.id.trim().is_empty()||self.version.trim().is_empty(){return Err(EngineError::RuntimeUnavailable("runtime id/version is empty".into()));}
  if self.executable.as_os_str().is_empty(){return Err(EngineError::RuntimeUnavailable("runtime executable is empty".into()));}
  if !self.executable.is_file(){return Err(EngineError::RuntimeUnavailable(format!("runtime executable does not exist: {}",self.executable.display())));}
  Ok(())
 }
 pub fn major_version(&self)->Option<u32>{
  let first=self.version.split('.').next()?.parse::<u32>().ok()?;
  if first==1 { self.version.split('.').nth(1)?.parse().ok() } else { Some(first) }
}
 pub fn supports_major(&self,required:u32)->bool{self.major_version()==Some(required)}
}

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct RuntimeRegistry { pub runtimes:Vec<JavaRuntime> }

impl RuntimeRegistry {
 pub fn register(&mut self,runtime:JavaRuntime)->Result<(),EngineError>{
  runtime.validate()?;
  self.runtimes.retain(|r|r.id!=runtime.id);
  self.runtimes.push(runtime);
  Ok(())
 }
 pub fn find(&self,id:&str)->Option<&JavaRuntime>{self.runtimes.iter().find(|r|r.id==id)}
 pub fn select_for_major(&self,major:u32)->Option<&JavaRuntime>{self.runtimes.iter().find(|r|r.supports_major(major))}
}

#[derive(Debug,Clone)]
pub struct RuntimeManager { pub registry:RuntimeRegistry }

impl RuntimeManager {
 pub fn new(registry:RuntimeRegistry)->Self{Self{registry}}
 pub fn discover_executable(&mut self,id:impl Into<String>,path:impl Into<PathBuf>)->Result<JavaRuntime,EngineError>{
  let path=path.into();
  let version=probe_java_version(&path)?;
  let runtime=JavaRuntime{id:id.into(),version,executable:path};
  self.registry.register(runtime.clone())?;
  Ok(runtime)
 }
 pub fn select(&self,required_major:Option<u32>,preferred_id:Option<&str>)->Result<&JavaRuntime,EngineError>{
  if let Some(id)=preferred_id { if let Some(r)=self.registry.find(id) { if required_major.map(|v|r.supports_major(v)).unwrap_or(true){return Ok(r);} } }
  if let Some(v)=required_major { self.registry.select_for_major(v).ok_or_else(||EngineError::RuntimeUnavailable(format!("no Java runtime for major version {v}"))) }
  else { self.registry.runtimes.first().ok_or_else(||EngineError::RuntimeUnavailable("no Java runtimes registered".into())) }
 }
}

pub fn probe_java_version(executable:&Path)->Result<String,EngineError>{
 let output=Command::new(executable).arg("-version").output().map_err(|e|EngineError::RuntimeUnavailable(format!("run java -version: {e}")))?;
 if !output.status.success(){return Err(EngineError::RuntimeUnavailable(format!("java -version exited with {}",output.status)));}
 let text=String::from_utf8_lossy(&output.stderr);
 parse_java_version(&text).ok_or_else(||EngineError::RuntimeUnavailable("could not parse Java version".into()))
}

fn parse_java_version(text:&str)->Option<String>{
 let marker=text.lines().find(|l|l.contains("version"))?;
 let start=marker.find('"')?+1;let end=marker[start..].find('"')?+start;Some(marker[start..end].to_string())
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn parses_java_versions(){assert_eq!(parse_java_version(r#"openjdk version "17.0.12" 2024-07-16"#),Some("17.0.12".into()));assert_eq!(parse_java_version(r#"java version "1.8.0_402""#),Some("1.8.0_402".into()));}
 #[test]fn registry_selects_required_major(){let mut r=RuntimeRegistry::default();let rt=JavaRuntime{id:"j17".into(),version:"17.0.12".into(),executable:PathBuf::from("/tmp/java")};r.runtimes.push(rt);assert_eq!(r.select_for_major(17).unwrap().id,"j17");}
}
#[cfg(test)]
mod major_version_tests {
 use super::*;
 use std::path::PathBuf;
 #[test] fn java_eight_uses_legacy_major(){let r=JavaRuntime{id:"j8".into(),version:"1.8.0_402".into(),executable:PathBuf::from("/tmp/java")};assert_eq!(r.major_version(),Some(8));}
 #[test] fn modern_java_uses_first_component(){let r=JavaRuntime{id:"j21".into(),version:"21.0.8".into(),executable:PathBuf::from("/tmp/java")};assert_eq!(r.major_version(),Some(21));}
}
