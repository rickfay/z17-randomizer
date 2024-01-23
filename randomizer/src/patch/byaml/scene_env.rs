use crate::patch::Patcher;
use log::info;
use modinfo::Settings;
use rom::byaml::scene_env::SceneEnvFile;

/// Patch the SceneEnv.byaml file
pub fn patch(patcher: &mut Patcher, settings: &Settings) -> Option<SceneEnvFile> {
    // TODO not using this for anything yet
    if !&settings.dev_mode {
        return None;
    }

    info!("Patching SceneEnv.byaml...");

    let mut scene_env_file = patcher.scene_env().expect("loading World/Byaml/SceneEnv.byaml");
    let _scene_env = scene_env_file.scene_env.get_mut();

    // (scene_env.IndoorLight1.get_mut(0).unwrap() as &mut SceneEnvScene).turn_off_lights();
    // (scene_env.FieldLight.get_mut(0).unwrap() as &mut SceneEnvScene).turn_off_lights();
    // (scene_env.FieldLight43.get_mut(0).unwrap() as &mut SceneEnvScene).turn_off_lights();

    Some(scene_env_file)
}
