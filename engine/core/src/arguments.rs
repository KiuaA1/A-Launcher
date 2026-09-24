use std::collections::HashMap;
use serde_json::Value;
use crate::{error::EngineError, launch::LaunchPlan, manifest::{Arguments, Rule}, plan::LaunchPreparation, platform::PlatformProfile};

#[derive(Debug, Clone, Default)]
pub struct LaunchContext {
    pub username:String,pub uuid:String,pub access_token:String,pub assets_root:String,pub asset_index:String,
    pub natives_directory:String,pub version_name:String,pub launcher_name:String,pub launcher_version:String,
    pub game_directory:String,pub classpath:String,pub feature_map:HashMap<String,bool>,
    pub platform_os:String,pub platform_arch:String,
}
impl LaunchContext {
    pub fn for_platform(profile:&PlatformProfile)->Self { Self { feature_map:profile.features.clone(), platform_os:profile.platform.os_name().into(), platform_arch:profile.platform.arch_name().into(), ..Self::default() } }

    pub fn substitute(&self,input:&str)->String {
        let mut out=input.to_string();
        for (k,v) in [("${auth_player_name}",&self.username),("${auth_uuid}",&self.uuid),("${auth_access_token}",&self.access_token),
            ("${assets_root}",&self.assets_root),("${asset_index_name}",&self.asset_index),("${natives_directory}",&self.natives_directory),
            ("${version_name}",&self.version_name),("${launcher_name}",&self.launcher_name),("${launcher_version}",&self.launcher_version),
            ("${game_directory}",&self.game_directory),("${classpath}",&self.classpath)] { out=out.replace(k,v); }
        out
    }
}
fn rule_matches(rule:&Rule,ctx:&LaunchContext)->bool {
    if let Some(os)=&rule.os {
        if let Some(name)=&os.name { let current=ctx.platform_os.as_str(); if name!=current{return false;} }
        if let Some(arch)=&os.arch { let current=ctx.platform_arch.as_str(); if arch!=current{return false;} }
    }
    true
}
fn rules_allow(rules:&[Rule],ctx:&LaunchContext)->bool { if rules.is_empty(){return true;} let mut allowed=false; for r in rules {if rule_matches(r,ctx){allowed=r.action=="allow";}} allowed }
fn values(v:&Value,ctx:&LaunchContext,out:&mut Vec<String>)->Result<(),EngineError>{
    match v {
        Value::String(s)=>{out.push(ctx.substitute(s));Ok(())},
        Value::Array(a)=>{for x in a{values(x,ctx,out)?;}Ok(())},
        Value::Object(o)=>{
            let rules=match o.get("rules"){Some(Value::Array(a))=>a.iter().map(|x|serde_json::from_value::<Rule>(x.clone()).map_err(|e|EngineError::ManifestInvalid(e.to_string()))).collect::<Result<Vec<_>,_>>()?,_=>Vec::new()};
            if !rules_allow(&rules,ctx){return Ok(())}
            o.get("value").map(|v|values(v,ctx,out)).unwrap_or_else(||Err(EngineError::ManifestInvalid("argument object has no value".into())))
        },
        _=>Err(EngineError::ManifestInvalid("unsupported argument value".into()))
    }
}
pub fn build_arguments(arguments:Option<&Arguments>,legacy:Option<&str>,ctx:&LaunchContext)->Result<(Vec<String>,Vec<String>),EngineError>{
    if let Some(a)=arguments {let mut j=Vec::new();let mut g=Vec::new();for v in &a.jvm{values(v,ctx,&mut j)?;}for v in &a.game{values(v,ctx,&mut g)?;}return Ok((j,g));}
    Ok((Vec::new(),legacy.unwrap_or_default().split_whitespace().map(|s|ctx.substitute(s)).filter(|s|!s.is_empty()).collect()))
}
pub fn build_launch_plan(prep:&LaunchPreparation,java_executable:&str,mut ctx:LaunchContext)->Result<LaunchPlan,EngineError>{
    ctx.version_name=prep.minecraft_version.clone();ctx.game_directory=prep.game_directory.to_string_lossy().into_owned();
    let (mut jvm,mut game)=build_arguments(None,None,&ctx)?;
    jvm.push("-cp".into());jvm.push(ctx.classpath.clone());game.push(prep.main_class.clone());
    let p=LaunchPlan{minecraft_version:prep.minecraft_version.clone(),java_executable:java_executable.into(),game_directory:ctx.game_directory.clone(),classpath:vec![ctx.classpath],jvm_args:jvm,game_args:game};p.validate()?;Ok(p)
}
#[cfg(test)]
mod tests{use super::*;#[test]fn substitutes(){let c=LaunchContext{username:"Kiua".into(),..Default::default()};assert_eq!(c.substitute("${auth_player_name}"),"Kiua");}
#[test]fn modern_values(){let a=Arguments{game:vec![Value::String("--demo".into()),serde_json::json!({"rules":[{"action":"allow"}],"value":["--name","${auth_player_name}"]})],jvm:vec![]};let c=LaunchContext{username:"Player".into(),..Default::default()};let(_,g)=build_arguments(Some(&a),None,&c).unwrap();assert_eq!(g,vec!["--demo","--name","Player"]);}}