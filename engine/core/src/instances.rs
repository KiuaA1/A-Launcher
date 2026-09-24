use serde::{Deserialize,Serialize};
use std::{fs,path::{Path,PathBuf}};
use crate::{error::EngineError,fs::StorageLayout,instance::Instance};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InstanceConfig{
 pub instance:Instance,
 #[serde(default)] pub java_runtime_id:Option<String>,
 #[serde(default)] pub memory_mb:Option<u32>,
 #[serde(default)] pub jvm_args:Vec<String>,
 #[serde(default)] pub game_args:Vec<String>,
}

#[derive(Debug,Clone)]
pub struct InstanceManager{pub storage:StorageLayout}

impl InstanceManager{
 pub fn new(storage:StorageLayout)->Self{Self{storage}}
 pub fn create(&self,id:&str,name:&str,version:&str,loader:Option<String>)->Result<Instance,EngineError>{
  validate_id(id)?;
  if name.trim().is_empty(){return Err(EngineError::InvalidLaunchPlan("instance name is empty".into()));}
  if version.trim().is_empty(){return Err(EngineError::InvalidLaunchPlan("Minecraft version is empty".into()));}
  let instance=Instance{id:id.into(),name:name.into(),game_directory:self.storage.instance_game_dir(id),minecraft_version:version.into(),loader};
  let config=InstanceConfig{instance:instance.clone(),java_runtime_id:None,memory_mb:None,jvm_args:Vec::new(),game_args:Vec::new()};
  self.save(&config)?;
  Ok(instance)
 }
 pub fn save(&self,config:&InstanceConfig)->Result<(),EngineError>{
  validate_id(&config.instance.id)?;
  let dir=self.storage.instance_dir(&config.instance.id);
  fs::create_dir_all(&dir).map_err(|e|EngineError::RuntimeUnavailable(format!("create instance directory: {e}")))?;
  fs::create_dir_all(&config.instance.game_directory).map_err(|e|EngineError::RuntimeUnavailable(format!("create game directory: {e}")))?;
  let path=dir.join("instance.json");
  let json=serde_json::to_vec_pretty(config).map_err(|e|EngineError::ManifestInvalid(format!("serialize instance: {e}")))?;
  fs::write(path,json).map_err(|e|EngineError::RuntimeUnavailable(format!("write instance metadata: {e}")))?;
  Ok(())
 }
 pub fn load(&self,id:&str)->Result<InstanceConfig,EngineError>{
  validate_id(id)?;
  let path=self.storage.instance_dir(id).join("instance.json");
  let bytes=fs::read(&path).map_err(|e|EngineError::RuntimeUnavailable(format!("read instance metadata: {e}")))?;
  serde_json::from_slice(&bytes).map_err(|e|EngineError::ManifestInvalid(format!("parse instance metadata: {e}")))
 }
 pub fn list(&self)->Result<Vec<Instance>,EngineError>{
  let mut out=Vec::new();
  let entries=fs::read_dir(&self.storage.instances).map_err(|e|EngineError::RuntimeUnavailable(format!("list instances: {e}")))?;
  for entry in entries{
   let entry=entry.map_err(|e|EngineError::RuntimeUnavailable(format!("read instance entry: {e}")))?;
   if entry.path().join("instance.json").is_file(){
    if let Ok(cfg)=self.load(&entry.file_name().to_string_lossy()){out.push(cfg.instance);}
   }
  }
  out.sort_by(|a,b|a.id.cmp(&b.id)); Ok(out)
 }
 pub fn delete(&self,id:&str)->Result<(),EngineError>{
  validate_id(id)?;
  let dir=self.storage.instance_dir(id);
  if dir.exists(){fs::remove_dir_all(dir).map_err(|e|EngineError::RuntimeUnavailable(format!("delete instance: {e}")))?;}
  Ok(())
 }
}

fn validate_id(id:&str)->Result<(),EngineError>{
 if id.trim().is_empty()||id=="."||id==".."||Path::new(id).components().any(|c|matches!(c,std::path::Component::ParentDir|std::path::Component::RootDir|std::path::Component::Prefix(_))){
  return Err(EngineError::InvalidLaunchPlan("invalid instance id".into()));
 }
 Ok(())
}
