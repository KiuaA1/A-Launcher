use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}};
use crate::{
 error::EngineError,
 fs::StorageLayout,
 instances::{InstanceConfig, InstanceManager},
 instance::Instance,
 LaunchEvent, LaunchState,
 mojang::MojangResolver,
 resolver::{maven_path, Resolution, TargetPlatform},
 download::DownloadTransport,
 classpath::{Classpath, build_classpath},
 native::extract_native_jar,
 artifacts::download_resolution as download_artifacts,
 plan::LaunchPreparation,
 runtime::RuntimeManager,
 manifest::VersionJson,
 arguments::LaunchContext,
 launch::LaunchPlan,
 process::{DefaultProcessManager, ManagedProcess, ProcessManager, ProcessEvent},
 session::{LaunchSession, LaunchSessionRegistry},
};

#[derive(Debug, Clone)]
pub struct LauncherEngine {
 pub storage: StorageLayout,
 pub instances: InstanceManager,
 sessions: Arc<Mutex<LaunchSessionRegistry>>,
 processes: Arc<Mutex<HashMap<String, ManagedProcess>>>,
}

impl LauncherEngine {
 pub fn new(root: impl Into<PathBuf>) -> Result<Self, EngineError> {
  let storage=StorageLayout::new(root);
  storage.ensure_dirs().map_err(|e| EngineError::RuntimeUnavailable(format!("initialize storage: {e}")))?;
  Ok(Self {
   instances: InstanceManager::new(storage.clone()),
   storage,
   sessions: Arc::new(Mutex::new(LaunchSessionRegistry::default())),
   processes: Arc::new(Mutex::new(HashMap::new())),
  })
 }

 pub fn event(state: LaunchState, message: impl Into<String>) -> LaunchEvent { LaunchEvent::new(state,message) }

 pub fn create_instance(&self,id:&str,name:&str,version:&str,loader:Option<String>)->Result<Instance,EngineError>{self.instances.create(id,name,version,loader)}
 pub fn instance_launch_config(&self,id:&str)->Result<InstanceConfig,EngineError>{self.instances.load(id)}
 pub fn load_instance(&self,id:&str)->Result<InstanceConfig,EngineError>{self.instances.load(id)}
 pub fn list_instances(&self)->Result<Vec<Instance>,EngineError>{self.instances.list()}
 pub fn delete_instance(&self,id:&str)->Result<(),EngineError>{self.instances.delete(id)}

 pub fn resolve_version<T:DownloadTransport>(&self,resolver:&MojangResolver,version:&str,transport:&T,platform:TargetPlatform)->Result<(Resolution,Vec<LaunchEvent>),EngineError>{
  let mut events=vec![Self::event(LaunchState::Resolving,format!("resolving Minecraft {version}"))];
  let resolution=resolver.resolve_with_transport(version,transport,&self.storage.cache,platform)?;
  events.push(Self::event(LaunchState::Resolving,format!("resolved Minecraft {version}: {} libraries, {} native libraries",resolution.libraries.len(),resolution.native_libraries.len())));
  Ok((resolution,events))
 }

 pub fn download_resolution<T:DownloadTransport>(&self,instance_id:&str,resolution:&Resolution,transport:&T)->Result<(usize,Vec<LaunchEvent>),EngineError>{
  let mut events=vec![Self::event(LaunchState::Downloading,"downloading resolved Minecraft artifacts")];
  let summary=download_artifacts(transport,&self.storage,instance_id,resolution)?;
  events.push(Self::event(LaunchState::Downloading,format!("verified {} artifacts ({} cached, {} downloaded)",summary.total(),summary.cached,summary.downloaded)));
  Ok((summary.total(),events))
 }

 pub fn prepare_runtime(&self,instance_id:&str,resolution:&Resolution)->Result<(),EngineError>{
  self.storage.ensure_dirs().map_err(|e|EngineError::RuntimeUnavailable(format!("prepare storage: {e}")))?;
  for artifact in &resolution.native_libraries {
   let rel=artifact.path.clone().or_else(||maven_path(&artifact.id,artifact.classifier.as_deref(),"jar").ok())
    .ok_or_else(||EngineError::InvalidLaunchPlan(format!("cannot derive native path for {}",artifact.id)))?;
   let jar=self.storage.libraries.join(&rel);
   if !jar.is_file(){return Err(EngineError::DownloadFailed(format!("native JAR missing: {}",jar.display())));}
   extract_native_jar(&jar,&self.storage.natives.join(instance_id))?;
  }
  Ok(())
 }

 pub fn build_launch_plan(&self,instance_id:&str,version:&VersionJson,resolution:Resolution,runtime:&RuntimeManager,context:LaunchContext)->Result<LaunchPlan,EngineError>{
  let config=self.instances.load(instance_id)?;
  validate_instance_launch(&self.storage, &config)?;
  if config.instance.minecraft_version!=version.id{return Err(EngineError::InvalidLaunchPlan("instance and version disagree".into()));}
  let prep0=LaunchPreparation::from_metadata(version,resolution.clone(),config.instance.game_directory.clone())?;
  let prep=LaunchPreparation{memory_mb:config.memory_mb,instance_jvm_args:config.jvm_args.clone(),instance_game_args:config.game_args.clone(),..prep0};
  let required=prep.java_major_version;
  let selected=runtime.select(required,config.java_runtime_id.as_deref())?;
  let client_path=self.storage.instance_game_dir(instance_id).join("client.jar");
  let classpath=build_classpath(&prep.artifacts.libraries,&prep.artifacts.client_jar,&self.storage.libraries,&client_path)?;
  let mut plan=prep.build_launch_plan(&selected.executable.to_string_lossy(),&classpath,version.arguments.as_ref(),version.minecraft_arguments.as_deref(),context)?;
  plan.environment=config.environment;
  Ok(plan)
 }

 pub fn launch(&self,session_id:&str,version:&str,plan:&LaunchPlan)->Result<(),EngineError>{
  if self.processes.lock().map_err(|_|EngineError::RuntimeUnavailable("process registry lock poisoned".into()))?.contains_key(session_id){
   return Err(EngineError::RuntimeUnavailable(format!("launch session already active: {session_id}")));
  }
  let process=DefaultProcessManager.spawn(plan)?;
  let mut sessions=self.sessions.lock().map_err(|_|EngineError::RuntimeUnavailable("session registry lock poisoned".into()))?;
  sessions.insert(LaunchSession::new(session_id,version));
  drop(sessions);
  self.processes.lock().map_err(|_|EngineError::RuntimeUnavailable("process registry lock poisoned".into()))?.insert(session_id.into(),process);
  Ok(())
 }

 pub fn poll_process_events(&self,session_id:&str)->Result<Vec<ProcessEvent>,EngineError>{
  let process=self.processes.lock().map_err(|_|EngineError::RuntimeUnavailable("process registry lock poisoned".into()))?
   .get(session_id).ok_or_else(||EngineError::RuntimeUnavailable(format!("process not found: {session_id}")))?;
  let events=process.drain_events();
  let mut sessions=self.sessions.lock().map_err(|_|EngineError::RuntimeUnavailable("session registry lock poisoned".into()))?;
  let session=sessions.get_mut(session_id).ok_or_else(||EngineError::RuntimeUnavailable(format!("launch session not found: {session_id}")))?;
  for event in &events { session.apply(event.clone()); }
  Ok(events)
 }

 pub fn session(&self,session_id:&str)->Result<Option<LaunchSession>,EngineError>{
  let sessions=self.sessions.lock().map_err(|_|EngineError::RuntimeUnavailable("session registry lock poisoned".into()))?;
  Ok(sessions.get(session_id).cloned())
 }

 pub fn terminate(&self,session_id:&str)->Result<(),EngineError>{
  let process=self.processes.lock().map_err(|_|EngineError::RuntimeUnavailable("process registry lock poisoned".into()))?
   .get(session_id).ok_or_else(||EngineError::RuntimeUnavailable(format!("process not found: {session_id}")))?;
  process.kill()?;
  let mut sessions=self.sessions.lock().map_err(|_|EngineError::RuntimeUnavailable("session registry lock poisoned".into()))?;
  if let Some(session)=sessions.get_mut(session_id){session.cancel();}
  Ok(())
 }

 pub fn remove_finished_process(&self,session_id:&str)->Result<Option<LaunchSession>,EngineError>{
  let _=self.poll_process_events(session_id)?;
  let session=self.session(session_id)?;
  if matches!(session.as_ref().map(|s|&s.status),Some(crate::session::SessionStatus::Exited|crate::session::SessionStatus::Failed|crate::session::SessionStatus::Cancelled)){
   self.processes.lock().map_err(|_|EngineError::RuntimeUnavailable("process registry lock poisoned".into()))?.remove(session_id);
  }
  Ok(session)
 }
}


fn validate_instance_launch(storage:&StorageLayout,config:&InstanceConfig)->Result<(),EngineError>{
 let game_dir=&config.instance.game_directory;
 if !game_dir.is_dir(){
  return Err(EngineError::InvalidLaunchPlan(format!("instance game directory is missing: {}",game_dir.display())));
 }
 if !game_dir.starts_with(storage.instance_dir(&config.instance.id)){
  return Err(EngineError::InvalidLaunchPlan("instance game directory escapes instance storage".into()));
 }
 let client=game_dir.join("client.jar");
 if !client.is_file(){
  return Err(EngineError::InvalidLaunchPlan(format!("client JAR is missing: {}",client.display())));
 }
 if let Some(memory)=config.memory_mb{
  if memory<256{return Err(EngineError::InvalidLaunchPlan("instance memory must be at least 256 MiB".into()));}
 }
 for arg in &config.jvm_args{
  if arg.starts_with("-Xmx")||arg=="-jar"||arg=="--class-path"||arg.starts_with("--class-path=")||arg=="-cp"{continue;}
 }
 Ok(())
}
