use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::{arguments::{build_arguments,LaunchContext},classpath::Classpath,error::EngineError,launch::LaunchPlan,manifest::VersionJson,resolver::{MinecraftArtifact,Resolution}};

#[derive(Debug,Clone,Serialize,Deserialize)] pub struct ResolvedLaunchArtifacts{pub client_jar:MinecraftArtifact,pub libraries:Vec<MinecraftArtifact>,pub native_libraries:Vec<MinecraftArtifact>}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct LaunchPreparation{pub minecraft_version:String,pub main_class:String,pub java_major_version:Option<u32>,pub artifacts:ResolvedLaunchArtifacts,pub game_directory:PathBuf,pub memory_mb:Option<u32>,pub instance_jvm_args:Vec<String>,pub instance_game_args:Vec<String>}
impl LaunchPreparation{
 pub fn from_metadata(version:&VersionJson,resolution:Resolution,game_directory:impl Into<PathBuf>)->Result<Self,EngineError>{
  if version.id!=resolution.minecraft_version{return Err(EngineError::InvalidLaunchPlan("version metadata and resolution disagree".into()));}
  if version.main_class.trim().is_empty(){return Err(EngineError::InvalidLaunchPlan("main class is empty".into()));}
  Ok(Self{minecraft_version:version.id.clone(),main_class:version.main_class.clone(),java_major_version:version.java_version.as_ref().map(|v|v.major_version),artifacts:ResolvedLaunchArtifacts{client_jar:resolution.client_jar,libraries:resolution.libraries,native_libraries:resolution.native_libraries},game_directory:game_directory.into(),memory_mb:None,instance_jvm_args:Vec::new(),instance_game_args:Vec::new()})
 }
 pub fn build_launch_plan(&self,java_executable:&str,classpath:&Classpath,arguments:&crate::manifest::Arguments,mut ctx:LaunchContext)->Result<LaunchPlan,EngineError>{
  ctx.version_name=self.minecraft_version.clone();ctx.game_directory=self.game_directory.to_string_lossy().into_owned();ctx.classpath=classpath.as_separator_string();
  let (mut jvm,resolved_game)=build_arguments(Some(arguments),None,&ctx)?;\n  if let Some(memory)=self.memory_mb {\n   if memory < 256 { return Err(EngineError::InvalidLaunchPlan("instance memory must be at least 256 MiB".into())); }\n   jvm.retain(|arg| !arg.starts_with("-Xmx"));\n   jvm.insert(0,format!("-Xmx{}M",memory));\n  }\n  for arg in &self.instance_jvm_args {\n   if arg.starts_with("-cp") || arg == "--class-path" || arg.starts_with("--class-path=") { continue; }\n   if arg.starts_with("-jar") { continue; }\n   if arg.starts_with("-Xmx") { continue; }\n   jvm.push(arg.clone());\n  }
  jvm.push("-cp".into());jvm.push(ctx.classpath.clone());\n  let mut game=Vec::with_capacity(resolved_game.len()+1);game.push(self.main_class.clone());game.extend(resolved_game);game.extend(self.instance_game_args.clone());
  let p=LaunchPlan{minecraft_version:self.minecraft_version.clone(),java_executable:java_executable.into(),game_directory:ctx.game_directory.clone(),classpath:classpath.entries.iter().map(|p|p.to_string_lossy().into_owned()).collect(),jvm_args:jvm,game_args:game};p.validate()?;Ok(p)
 }
}