use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use crate::resolver::{TargetArch,TargetOs,TargetPlatform};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PlatformProfile {
 pub platform:TargetPlatform,
 pub features:HashMap<String,bool>,
}

impl PlatformProfile {
 pub fn android_arm64()->Self {
  let mut features=HashMap::new();
  features.insert("is_demo_user".into(),false);
  features.insert("has_custom_resolution".into(),false);
  features.insert("has_quick_play_support".into(),false);
  features.insert("is_android".into(),true);
  Self{platform:TargetPlatform{os:TargetOs::Linux,arch:TargetArch::Arm64},features}
 }
 pub fn with_feature(mut self,name:impl Into<String>,enabled:bool)->Self{self.features.insert(name.into(),enabled);self}
}
