use std::path::PathBuf;
use crate::{error::EngineError,fs::StorageLayout,instances::InstanceManager,instance::Instance,LaunchEvent,LaunchState,mojang::MojangResolver,resolver::{Resolution,TargetPlatform},download::{DownloadRequest,DownloadTransport,prepare_download},classpath::{Classpath,build_classpath},native::extract_native_jar,plan::LaunchPreparation,runtime::RuntimeManager,manifest::VersionJson,arguments::LaunchContext,launch::LaunchPlan,process::{DefaultProcessManager,ManagedProcess,ProcessManager,ProcessEvent};

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
  let mut total=0;
  let client_path=self.storage.libraries.join(resolution.client_jar.path.as_deref().unwrap_or("clients/client.jar"));
  let artifacts=std::iter::once(&resolution.client_jar).chain(resolution.libraries.iter()).chain(resolution.native_libraries.iter());
  for artifact in artifacts {
   let rel=artifact.path.as_deref().ok_or_else(||EngineError::DownloadFailed(format!("artifact {} has no repository path",artifact.id)))?;
   let destination=self.storage.libraries.join(rel);
   let status=prepare_download(transport,&DownloadRequest{url:artifact.url.clone(),destination,expected_sha1:artifact.sha1.clone(),expected_size:artifact.size})?;
   let _=status; total+=1;
  }
  let _=client_path;
  events.push(Self::event(LaunchState::Downloading,format!("verified {total} artifacts")));
  Ok((total,events))
 }

 pub fn prepare_runtime(&self,resolution:&Resolution)->Result<(Classpath,usize,Vec<LaunchEvent>),EngineError>{
  let mut events=vec![Self::event(LaunchState::Preparing,"preparing runtime classpath and native libraries")];
  let client_rel=resolution.client_jar.path.as_deref().ok_or_else(||EngineError::InvalidLaunchPlan("client artifact has no path".into()))?;
  let client_path=self.storage.libraries.join(client_rel);
  let classpath=build_classpath(&resolution.libraries,&resolution.client_jar,&self.storage.libraries,&client_path)?;
  let native_dir=self.storage.natives.join(&resolution.minecraft_version);
  let mut extracted=0;
  for native in &resolution.native_libraries {
   let rel=native.path.as_deref().ok_or_else(||EngineError::InvalidLaunchPlan(format!("native {} has no path",native.id)))?;
   extracted+=extract_native_jar(&self.storage.libraries.join(rel),&native_dir)?;
  }
  events.push(Self::event(LaunchState::Preparing,format!("runtime ready: {} classpath entries, {extracted} native files",classpath.entries.len())));
  Ok((classpath,extracted,events))
 }

 pub fn build_launch_plan(&self,version:&VersionJson,resolution:Resolution,runtime:&RuntimeManager,classpath:Classpath,game_directory:impl Into<PathBuf>,context:LaunchContext)->Result<LaunchPlan,EngineError>{
  let prep=LaunchPreparation::from_metadata(version,resolution,game_directory)?;
  let required=prep.java_major_version;
  let selected=runtime.select(required,None)?;
  prep.build_launch_plan(&selected.executable.to_string_lossy(),&classpath,version.arguments.as_ref(),version.minecraft_arguments.as_deref(),context)
 }

 pub fn launch(&self,plan:&LaunchPlan)->Result<(ManagedProcess,Vec<LaunchEvent>),EngineError>{
  let manager=DefaultProcessManager;
  manager.validate_launch(plan)?;
  let process=manager.spawn(plan)?;
  let events=vec![Self::event(LaunchState::Starting,"Minecraft process started"),Self::event(LaunchState::Running,"Minecraft is running")];
  Ok((process,events))
 }

 pub fn poll_process_events(&self,process:&ManagedProcess)->Vec<LaunchEvent>{
  process.events().try_iter().map(|event| match event {
   ProcessEvent::Started=>Self::event(LaunchState::Starting,"process stream connected"),
   ProcessEvent::Stdout(line)=>Self::event(LaunchState::Running,line),
   ProcessEvent::Stderr(line)=>Self::event(LaunchState::Running,line),
   ProcessEvent::Exited(code)=>{ let mut e=LaunchEvent::exited(code); if code!=0 { e.state=LaunchState::Failed; } e }
  }).collect()
 }

 pub fn prepare(&self)->Result<LaunchEvent,EngineError>{
  self.storage.ensure_dirs().map_err(|e|EngineError::RuntimeUnavailable(format!("prepare storage: {e}")))?;
  Ok(Self::event(LaunchState::Preparing,"engine storage is ready"))
 }
}
