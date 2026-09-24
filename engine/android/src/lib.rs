use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use a_launcher_core::engine::LauncherEngine;
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
    match a_launcher_core::engine::validate_instance_launch(&engine.storage, &config) {
        Ok(()) => 1,
        Err(_) => 0,
    }
}
