use bevy::{
      prelude::*,
      render::texture::{ImageLoaderSettings, ImageSampler},
      utils::HashMap,
  };
  
  pub(super) fn plugin(app: &mut App) {
 
      app.register_type::<HandleMap<FontKey>>();
      app.init_resource::<HandleMap<FontKey>>();
  }

  #[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum FontKey {
    GeoFont,
}

impl AssetKey for FontKey {
    type Asset = Font;
}

impl FromWorld for HandleMap<FontKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            FontKey::GeoFont,
            asset_server.load("fonts/GeoFont-Bold.otf"),
        )]
        .into()
    }
}


pub trait AssetKey: Sized {
      type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
