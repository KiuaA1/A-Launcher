use std::path::PathBuf;
use crate::{error::EngineError,fs::StorageLayout,instances::InstanceManager,instance::Instance,LaunchEvent,LaunchState,mojang::MojangResolver,resolver::{maven_path,Resolution,TargetPlatform},download::DownloadTransport,classpath::{Classpath,build_classpath},native::extract_native_jar,artifacts::download_resolution as download_artifacts,plan::LaunchPreparation,runtime::RuntimeManager,manifest::VersionJson,arguments::LaunchContext,launch::LaunchPlan,process::{DefaultProcessManager,ManagedProcess,ProcessManager,ProcessEvent};

#[derive(Debug,Clone)]
pub struct LauncherEngine { pub storage:StorageLayout, pub instances:InstanceManager }

impl LauncherEngine {
 pub fn new(root:impl Into<PathBuf>)->Result<Self,EngineError>{
  let storage=StorageLayout::new(root);
  storage.ensure_dirs().map_err(|e|EngineError::RuntimeUnavailable(format!("initialize storage: {e}")))?;
  Ok(Self{instances:InstanceManager::new(storage.clone()),storage})
 }
 pub fn event(state:LaunchState,message:impl Into<String>)->LaunchEvent{LaunchEvent::new(state,message)}
 pub fn create_instance(&self,id:&str,name:&str,version:&str,loader:Option<String>)->Result<Instance,EngineError>{ self.instances.create(id,name,version,loader) }\n\n pub fn instance_launch_config(&self,id:&str)->Result<crate::instances::InstanceConfig,EngineError>{ self.instances.load(id) }\n\n pub fn load_instance(&self,id:&str)->Result<crate::instances::InstanceConfig,EngineError>{ self.instances.load(id) }\n\n pub fn list_instances(&self)->Result<Vec<Instance>,EngineError>{ self.instances.list() }\n\n pub fn delete_instance(&self,id:&str)->Result<(),EngineError>{ self.instances.delete(id) }\n\n pub fn resolve_version<T: DownloadTransport>(&self,resolver:&MojangResolver,version:&str,transport:&T,platform:TargetPlatform)->Result<(Resolution,Vec<LaunchEvent>),EngineError>{
  let mut events=vec![Self::event(LaunchState::Resolving,format!("resolving Minecraft {version}"))];
  let metadata_root=&self.storage.cache;
  let resolution=resolver.resolve_with_transport(version,transport,metadata_root,platform)?;
  events.push(Self::event(LaunchState::Resolving,format!("resolved Minecraft {version}: {} libraries, {} native libraries",resolution.libraries.len(),resolution.native_libraries.len())));
  Ok((resolution,events))
 }

 pub fn download_resolution<T: DownloadTransport>(&self,resolution:&Resolution,transport:&T)->Result<(usize,Vec<LaunchEvent>),EngineError>{
  let mut events=vec![Self::event(LaunchState::Downloading,"downloading resolved Minecraft artifacts")];
  let summary=download_artifacts(transport,&self.storage,resolution)?;
  events.push(Self::event(LaunchState::Downloading,format!("verified {} artifacts ({} cached, {} downloaded)",summary.total(),summary.cached,summary.downloaded)));
  Ok((summary.total(),events))
 }
