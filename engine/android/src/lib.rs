use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use a_launcher_core::engine::LauncherEngine;
use a_launcher_core::{arguments::LaunchContext, download::ReqwestDownloadTransport, mojang::{MojangMetadataClient, MojangResolver}, platform::PlatformProfile, runtime::{JavaRuntime, RuntimeManager, RuntimeRegistry}};
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

static ENGINE: OnceLock<Mutex<Option<LauncherEngine>>> = OnceLock::new();

fn engine_slot() -> &'static Mutex<Option<LauncherEngine>> {
    ENGINE.get_or_init(|| Mutex::new(None))
}

fn jstring_value(env: &mut JNIEnv, value: JString) -> Result<String, String> {
    env.get_string(&value)
        .map(|v| v.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

#[no_mangle]
pub extern "system" fn Java_com_kiuaa1_alauncher_NativeLauncherBridgeImpl_nativeInit(
    mut env: JNIEnv,
    _class: JClass,
    root: JString,
) -> jni::sys::jint {
    let root = match jstring_value(&mut env, root) {
        Ok(v) => v,
        Err(_) => return -1,
    };
    match LauncherEngine::new(PathBuf::from(root)) {
        Ok(engine) => {
            let mut slot = engine_slot().lock().unwrap();
            *slot = Some(engine);
            0
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_kiuaa1_alauncher_NativeLauncherBridgeImpl_nativeEngineVersion(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let version = a_launcher_core::ENGINE_VERSION;
    match env.new_string(version) {
        Ok(value) => value.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_kiuaa1_alauncher_NativeLauncherBridgeImpl_nativeListInstances(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let result = {
        let java = match jstring_value(&mut env, java_executable) { Ok(v) => v, Err(_) => return 0 };
    let slot = engine_slot().lock().unwrap();
        match slot.as_ref() {
            Some(engine) => match engine.list_instances() {
                Ok(instances) => serde_json::to_string(
                    &instances
                        .into_iter()
                        .map(|i| serde_json::json!({
                            "id": i.id,
                            "name": i.name,
                            "minecraftVersion": i.minecraft_version,
                            "loader": i.loader,
                        }))
                        .collect::<Vec<_>>(),
                )
                .unwrap_or_else(|_| "[]".to_string()),
                Err(_) => "[]".to_string(),
            },
            None => "[]".to_string(),
        }
    };
    match env.new_string(result) {
        Ok(value) => value.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_kiuaa1_alauncher_NativeLauncherBridgeImpl_nativeLaunch(
    mut env: JNIEnv,
    _class: JClass,
    instance_id: JString,
    java_executable: JString,
) -> jni::sys::jint {
    let id = match jstring_value(&mut env, instance_id) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let slot = engine_slot().lock().unwrap();
    let Some(engine) = slot.as_ref() else { return 0; };
    let config = match engine.instance_launch_config(&id) {
        Ok(config) => config,
        Err(_) => return 0,
    };
    if a_launcher_core::engine::validate_instance_launch(&engine.storage, &config).is_err() {
        return 0;
    }
    if java.is_empty() { return 0; }
    let transport = match ReqwestDownloadTransport::new() { Ok(value) => value, Err(_) => return 0 };
    let metadata = MojangMetadataClient::new(transport);
    let manifest_path = MojangMetadataClient::<ReqwestDownloadTransport>::cached_manifest_path(&engine.storage.cache);
    if !manifest_path.exists() && metadata.download_manifest(&manifest_path).is_err() { return 0; }
    let manifest = match std::fs::read_to_string(&manifest_path) { Ok(value) => value, Err(_) => return 0 };
    let resolver = match MojangResolver::from_manifest_json(manifest) { Ok(value) => value, Err(_) => return 0 };
    let resolution = match resolver.resolve_with_transport(
        &config.instance.minecraft_version,
        &metadata.transport,
        &engine.storage.cache,
        a_launcher_core::resolver::TargetPlatform::android_arm64(),
    ) { Ok(value) => value, Err(_) => return 0 };
    if engine.download_resolution(&id, &resolution, &metadata.transport).is_err() { return 0; }
    if engine.prepare_runtime(&id, &resolution).is_err() { return 0; }
    let version = match resolver.resolve_inheritance_chain(
        &config.instance.minecraft_version,
        &metadata.transport,
        &engine.storage.cache,
    ) { Ok(value) => value, Err(_) => return 0 };
    let path = PathBuf::from(java);
    let version_string = match a_launcher_core::runtime::probe_java_version(&path) { Ok(v) => v, Err(_) => return 0 };
    let mut registry = RuntimeRegistry::default();
    if registry.register(JavaRuntime { id: "android-selected".into(), version: version_string, executable: path }).is_err() { return 0; }
    let runtime = RuntimeManager::new(registry);
    let context = LaunchContext::for_platform(&PlatformProfile::android_arm64());
    let plan = match engine.build_launch_plan(&id, &version, resolution, &runtime, context) { Ok(p) => p, Err(_) => return 0 };
    match engine.launch(&id, &version.id, &plan) { Ok(()) => 3, Err(_) => 0 }
}
