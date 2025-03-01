use std::collections::HashSet;
use bevy::asset::{LoadState, UntypedAssetId};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Default)]
pub struct LoadingStateGroups {
    pub groups: HashMap<String, HashMap<UntypedAssetId, UntypedHandle>>,
    pub loading_states: HashMap<UntypedAssetId, HashSet<String>>
}

impl LoadingStateGroups {
    pub fn is_loaded(&self, key: String) -> bool {
        match self.groups.get(&key) {
            Some(group) => group.is_empty(),
            None => true
        }
    }

    pub fn add_asset(&mut self, key: String, handle: UntypedHandle) {
        match self.groups.get_mut(&key) {
            Some(group) => { group.insert(handle.id(), handle.clone()); }
            None => _ = {
                let mut hashmap = HashMap::<UntypedAssetId, UntypedHandle>::new();
                hashmap.insert(handle.id(), handle.clone());
                self.groups.insert(key.clone(), hashmap);
            }
        }

        match self.loading_states.get_mut(&handle.id()) {
            Some(set) => {
                set.insert(key);
            }
            None => {
                let mut hashset = HashSet::<String>::new();
                hashset.insert(key);
                let mut hashmap = HashMap::<UntypedAssetId, HashSet<String>>::new();
                hashmap.insert(handle.id(), hashset);
            }
        }
    }

    pub fn remove_asset(&mut self, id: &UntypedAssetId) {
        if let Some(groups_to_remove) = self.loading_states.remove(id) {
            for key in groups_to_remove {
                if let Some(group) = self.groups.get_mut(&key) {
                    group.remove(id);
                    if group.is_empty() {
                        self.groups.remove(&key);
                    }
                }
            }
        }

        // TODO : add events.
    }

    pub fn update_with_asset_server(&mut self, asset_server: &AssetServer) {
        for id in self.loading_states.keys().cloned().collect::<Vec<_>>() {
            if let Some(state) = asset_server.get_load_state(id.clone()) {
                match state {
                    LoadState::Loaded => {
                        self.remove_asset(&id);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct AppLoadingStatePlugin;

impl Plugin for AppLoadingStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingStateGroups>();
    }
}