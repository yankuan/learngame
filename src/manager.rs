use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::utils::HashSet;
use bevy::{render::primitives::Aabb, sprite::Mesh2dHandle, utils::hashbrown::HashMap};
use crate::consts::*;
use crate::math::*;
use crate::macros::*;
use crate::game::*;
use crate::mat::*;




#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationSet;

#[derive(Clone, Reflect, Debug)]
pub struct SpriteInfo {
    pub path: String,
    pub size: UVec2,
    pub color: Color,
}

impl Default for SpriteInfo {
    fn default() -> Self {
        Self {
            path: "sprites/default.png".into(),
            size: UVec2::new(1, 1),
            color: Color::WHITE,
        }
    }
}
impl SpriteInfo {
    pub fn new(path: &str, width: u32, height: u32) -> Self {
        Self {
            path: path.into(),
            size: UVec2::new(width, height),
            color: Color::WHITE,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Clone, Reflect, Debug)]
pub struct AnimationNode {
    pub sprite: SpriteInfo,
    pub length: u32,
    pub fps: f32,
    pub next: Option<String>,
}
impl Default for AnimationNode {
    fn default() -> Self {
        Self {
            sprite: default(),
            length: 1,
            fps: DEFAULT_ANIMATION_FPS,
            next: None,
        }
    }
}
impl AnimationNode {
    pub fn new_static(sprite: SpriteInfo) -> Self {
        Self {
            sprite,
            length: 1,
            fps: DEFAULT_ANIMATION_FPS,
            next: None,
        }
    }

    pub fn new_dynamic(sprite: SpriteInfo, length: u32) -> Self {
        Self {
            sprite,
            length,
            fps: DEFAULT_ANIMATION_FPS,
            next: None,
        }
    }

    pub fn with_fps(mut self, fps: f32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_next(mut self, next: String) -> Self {
        self.next = Some(next);
        self
    }
}

#[derive(Default, Clone, PartialEq, Eq, Reflect, Debug)]
pub enum AnimationGrowth {
    #[default]
    Repeat,
    Stretch,
}

/// Marks an animation as being stable and not having a change that requires updating the mesh/mat/whatever
#[derive(Component, Default)]
struct AnimationStable;


#[derive(Component, Default, Clone, Reflect)]
struct AnimationData {
/// Seconds per frame
      spf: f32,
      length: u32,
      scroll: Vec2,
      flip_x: bool,
      flip_y: bool,
}

#[derive(Component, Default, Reflect)]
struct AnimationMap {
    handle_map: HashMap<String, Handle<Image>>,
}

/// Ix only increases once steps >= AnimationPace.0
#[derive(Component, Default, Clone, Reflect)]
struct AnimationIndex {
    ix: u32,
    time: f32,
}

/// Controls a single animation map. Use MultiAnimationManager to actually put it in a component.
#[derive(Clone, Reflect, Debug)]
pub struct AnimationManager {
    /// The animation nodes provided by this manager
    pub map: HashMap<String, AnimationNode>,
    // State that can be manipulated
    key: String,
    points: Vec<Vec2>,
    growth: AnimationGrowth,
    offset: Vec3,
    scale: Vec2,
    render_layers: RenderLayers,
    hidden: bool,
    scroll: Vec2,
    flip_x: bool,
    flip_y: bool,
    // Internal stuff
    /// The entity that actually has the mesh and everything for this manager
    body_eid: Entity,
    /// Used when changing animations, so they restart from the beginning
    force_index: Option<u32>,
}

macro_rules! impl_animation_manager_field {
    ($field:ident, $type:ty) => {
        paste::paste! {
            pub fn [<get_ $field>](&self) -> $type {
                self.$field.clone()
            }

            pub fn [<set_ $field>](&mut self, val: impl Into<$type>, commands: &mut Commands) {
                let val = val.into();
                if val == self.$field {
                    // Do nothing. Use reset to force `is_changed`.
                    return;
                }
                self.$field = val;
                if let Some(mut commands) = commands.get_entity(self.body_eid) {
                    commands.remove::<AnimationStable>();
                }
            }

            pub fn [<reset_ $field>](&mut self, val: impl Into<$type>, commands: &mut Commands) {
                self.$field = val.into();
                if let Some(mut commands) = commands.get_entity(self.body_eid) {
                    commands.remove::<AnimationStable>();
                }
            }

            pub fn [<with_ $field>](mut self, val: $type) -> Self {
                self.$field = val;
                self
            }
        }
    };
}

/// Implements state get/set api
impl AnimationManager {
    pub fn current_node(&self) -> AnimationNode {
        self.map.get(&self.key).unwrap().clone()
    }

    impl_animation_manager_field!(key, String);
    impl_animation_manager_field!(points, Vec<Vec2>);
    impl_animation_manager_field!(growth, AnimationGrowth);
    impl_animation_manager_field!(offset, Vec3);
    impl_animation_manager_field!(scale, Vec2);
    impl_animation_manager_field!(render_layers, RenderLayers);
    impl_animation_manager_field!(hidden, bool);
    impl_animation_manager_field!(scroll, Vec2);
    impl_animation_manager_field!(flip_x, bool);
    impl_animation_manager_field!(flip_y, bool);

    /// Resets the key and automatically sets the points to match the new sprite
    pub fn reset_key_with_points(&mut self, val: impl Into<String>, commands: &mut Commands) {
        let key: String = val.into();
        self.reset_key(key.clone(), commands);
        let size = self.map.get(&key).unwrap().sprite.size;
        let points = simple_rect(size.x as f32, size.y as f32);
        self.reset_points(points, commands);
        self.reset_force_index(0);
    }
    pub fn reset_force_index(&mut self, ix: u32) {
        self.force_index = Some(ix);
    }

}

/// Helpful functions for constructing AnimationManagers
impl AnimationManager {
    /// An animation with one key, with one frame
    pub fn single_static(sprite: SpriteInfo) -> Self {
        let node = AnimationNode {
            sprite: sprite.clone(),
            length: 1,
            ..default()
        };
        let mut map = HashMap::new();
        map.insert(sprite.path.clone(), node);
        Self {
            key: sprite.path.clone(),
            map,
            points: simple_rect(sprite.size.x as f32, sprite.size.y as f32),
            ..default()
        }
    }

    /// An animation with one key, and multiple frames
    pub fn single_repeating(sprite: SpriteInfo, length: u32) -> Self {
        let node = AnimationNode {
            sprite: sprite.clone(),
            length,
            ..default()
        };
        let mut map = HashMap::new();
        map.insert(sprite.path.clone(), node);
        Self {
            key: sprite.path.clone(),
            map,
            points: simple_rect(sprite.size.x as f32, sprite.size.y as f32),
            ..default()
        }
    }


    /// Creates an animation manager with the provided nodes and labels.
    /// The first node in the list will be the initial animation
    pub fn from_nodes(node_info: Vec<(&str, AnimationNode)>) -> Self {
        let mut map = HashMap::new();
        let first = node_info[0].clone();
        for (key, node) in node_info {
            map.insert(key.to_string(), node);
        }
        let points = simple_rect(first.1.sprite.size.x as f32, first.1.sprite.size.y as f32);
        Self {
            key: first.0.to_string(),
            map,
            points,
            ..default()
        }
    }
}
impl Default for AnimationManager {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            key: String::new(),
            points: vec![],
            growth: AnimationGrowth::Repeat,
            offset: Vec3::ZERO,
            scale: Vec2::ONE,
            render_layers: SpriteCamera::render_layers(),
            hidden: false,
            body_eid: Entity::PLACEHOLDER,
            force_index: Some(0),
            scroll: Vec2::ZERO,
            flip_x: false,
            flip_y: false,
        }
    }
}


/// A multianimation is _unstable_ when a key has been added or deleted. Otherwise,
/// it will have this component to mark it's stable, and updates will happen within
/// the `AnimationManager`s contained in the map.
#[derive(Component)]
pub struct MultiAnimationStable;

#[derive(Component, Default, Clone, Reflect)]
pub struct MultiAnimationManager {
    pub map: HashMap<String, AnimationManager>,
}
impl MultiAnimationManager {
    /// Constructs a new multi animation manager from a single animation manager
    pub fn from_single(single: AnimationManager) -> Self {
        let mut map = HashMap::new();
        map.insert("core".to_string(), single);
        Self { map }
    }

    /// Constructs a new multi animation manager given a list of animation managers
    pub fn from_pairs(pairs: Vec<(&str, AnimationManager)>) -> Self {
        let mut map = HashMap::new();
        for (key, manager) in pairs {
            map.insert(key.to_string(), manager);
        }
        Self { map }
    }


    /// Makes a bordered mesh
    pub fn bordered_mesh(
        points: Vec<Vec2>,
        inner: SpriteInfo,
        outer: SpriteInfo,
        width: f32,
    ) -> Self {
        let inner =
            AnimationManager::single_static(inner).with_points(outline_points(&points, -width));
        let outer = AnimationManager::single_static(outer)
            .with_points(points.clone())
            .with_offset(-Vec3::Z);
        let mut map = HashMap::new();
        map.insert("inner".to_string(), inner);
        map.insert("outer".to_string(), outer);

        Self { map }
    }

    pub fn single(&self) -> &AnimationManager {
        // NOTE: This doesn't actually check if there are multiple, slightly different from bevy convention
        self.map.values().next().unwrap()
    }
    

    pub fn single_mut(&mut self) -> &mut AnimationManager {
        // NOTE: This doesn't actually check if there are multiple, slightly different from bevy convention
        self.map.values_mut().next().unwrap()
    }

    pub fn manager(&self, key: &str) -> &AnimationManager {
        self.map.get(key).unwrap()
    }

    pub fn manager_mut(&mut self, key: &str) -> &mut AnimationManager {
        self.map.get_mut(key).unwrap()
    }
}

#[derive(Component, Default, Reflect)]
struct AnimationBodyMarker {
    key: String,
}

#[derive(Bundle, Default)]
struct AnimationBundle {
      name: Name,
      marker: AnimationBodyMarker,
      map: AnimationMap,
      index: AnimationIndex,
      pace: AnimationData,
      render_layers: RenderLayers,
      mesh: Mesh2dHandle,
      material: Handle<AnimationMaterial>,
      spatial: SpatialBundle,
  }

/// Looks for AnimationManagers that don't have AnimationBody and spawns them
fn stabilize_multi_animations(
      mut commands: Commands,
      mut multis: Query<
          (Entity, &mut MultiAnimationManager, Option<&Children>),
          Without<MultiAnimationStable>,
      >,
      bodies: Query<Entity, With<AnimationBodyMarker>>,
  ) {
       
      for (eid, mut multi, children) in multis.iter_mut() {
            println!("bbbbbbbbbbbb");
          if let Some(children) = children {
              for child in children.iter() {
                  if bodies.contains(*child) {
                      commands.entity(*child).despawn_recursive();
                  }
              }
          }
          let mut fresh_eid_map = HashMap::new();
          for key in multi.map.keys() {
              let mut fresh_eid = Entity::PLACEHOLDER;
              commands.entity(eid).with_children(|parent| {
                  fresh_eid = parent
                      .spawn(AnimationBundle {
                          name: Name::new(format!("animation_body_{}", key)),
                          marker: AnimationBodyMarker { key: key.clone() },
                          ..default()
                      })
                      .id();
                  fresh_eid_map.insert(key.clone(), fresh_eid);
              });
          }
          for (key, val) in fresh_eid_map.iter() {
              let manager = multi.manager_mut(key);
              manager.body_eid = *val;
          }
          commands.entity(eid).insert(MultiAnimationStable);
      }
  }

/// Marks an animation body as not needing to be queried in `play_animations`
#[derive(Component)]
struct AnimationStatic;

fn update_animation_bodies(
      mut commands: Commands,
      multis: Query<(Entity, &MultiAnimationManager)>,
      mut bodies: Query<
          (
              Entity,
              &Parent,
              &AnimationBodyMarker,
              &mut AnimationMap,
              &mut AnimationData,
              &mut Transform,
              &mut Visibility,
              &Mesh2dHandle,
              &Handle<AnimationMaterial>,
          ),
          Without<AnimationStable>,
      >,
      asset_server: Res<AssetServer>,
      mut mats: ResMut<Assets<AnimationMaterial>>,
      mut meshes: ResMut<Assets<Mesh>>,
  ) {
      let mut killed_mids = HashSet::new();
      for (
            eid,
            parent,
            body,
            mut map,
            mut data,
            mut tran,
            mut vis,
            old_mesh_handle,
            old_mat_handle,
        ) in bodies.iter_mut()
        {
            let (manager, current_node) = {
                  let Ok((mid, multi)) = multis.get(parent.get()) else {
                      continue;
                  };
    
                  let Some(manager) = multi.map.get(&body.key) else {
                      continue;
                  };
                  if manager.key.as_str() == "despawn" {
                        commands.entity(mid).despawn_recursive();
                        killed_mids.insert(mid);
                        continue;
                  }
                  let current_node = manager.current_node();
                  (manager, current_node)
            };
           
              // Update the handle map
            for (key, node) in manager.map.iter() {
            
                  if !map.handle_map.contains_key(key) {
                        println!("{:?}",&node.sprite.path);
                        let handle = asset_server.load(&node.sprite.path);
                        map.handle_map.insert(key.clone(), handle);
                  }
            }
            // Update the visibility
            *vis = if manager.hidden {
                  Visibility::Hidden
            } else {
                  Visibility::Inherited
            };
            // Update the data, remembering to add/remove `AnimationStatic` as needed to save us effort
            // NOTE: We also add AnimationStatic whenever this node is hidden to avoid work
            data.length = current_node.length;
            data.spf = 1.0 / current_node.fps;
            data.scroll = manager.scroll;
            if (data.length == 1 || manager.hidden)
                  && !(data.scroll.length_squared() > 0.0)
                  && (data.flip_x == manager.flip_x)
            {
                  //println!("1---------");
                  commands.entity(eid).insert(AnimationStatic);
            } else {
                  //println!("2---------");
                  commands.entity(eid).remove::<AnimationStatic>();
            }


            data.flip_x = manager.flip_x;
            data.flip_y = manager.flip_y;
            // Update the translation and scale
            tran.translation = manager.offset;
            if data.flip_x {
                  tran.translation.x *= -1.0;
            }
            if data.flip_y {
                  tran.translation.y *= -1.0;
            }
            tran.scale.x = manager.scale.x;
            tran.scale.y = manager.scale.y;
            // Redo the mesh
            let mesh_size = uvec2_bound(&manager.points);
            let x_rep = mesh_size.x as f32 / current_node.sprite.size.x as f32;
            let y_rep = mesh_size.y as f32 / current_node.sprite.size.y as f32;
            let image_handle = map.handle_map.get(&manager.key).unwrap().clone();
            let mat = AnimationMaterial::from_handle(
                  image_handle,
                  data.length,
                  Vec2::new(x_rep, y_rep),
                  current_node.sprite.color,
            );
            let mat_ass = mats.add(mat);
            // TODO: I can see a world where points live on the node, not the manager
            // Then we might be able to cache the work of making these meshes? Idk how much mem that would take
            let mesh = points_to_mesh(&manager.points, &mut meshes);
            commands.entity(eid).insert(mat_ass);
            commands.entity(eid).insert(mesh);
            commands.entity(eid).remove::<Aabb>(); // Makes the engine recalculate the Aabb so culling is right

            // Remove the old mat
            mats.remove(old_mat_handle.id());
            meshes.remove(old_mesh_handle.id());

            // Finally mark this animation as stable
            commands.entity(eid).insert(AnimationStable);
      }
  }

  
/// Actually play the animations. Happens during the FixedUpdate step.
fn play_animations(
      mut multis: Query<&mut MultiAnimationManager>,
      mut bodies: Query<
          (
              &Parent,
              &AnimationBodyMarker,
              &Handle<AnimationMaterial>,
              &mut AnimationIndex,
              &AnimationData,
          ),
          Without<AnimationStatic>,
      >,
      mut mats: ResMut<Assets<AnimationMaterial>>,
      time: Res<Time>,
      bullet_time: Res<BulletTime>,
      mut commands: Commands,
  ) {
      let time_factor = time.delta_seconds() * bullet_time.factor();
      for (parent, body, mat_handle, mut index, data) in bodies.iter_mut() {
          let Ok(mut multi) = multis.get_mut(parent.get()) else {
              continue;
          };
          let manager = multi.manager_mut(&body.key);
          let current_node = manager.current_node();
          let Some(mat) = mats.get_mut(mat_handle.id()) else {
              return;
          };
          // Zeroth, update the index if needed
          if let Some(forced) = manager.force_index {
              index.ix = forced;
              index.time = 0.0;
              manager.force_index = None;
          }
          // First update the material
          mat.ix_length_flipx_flipy[0] = index.ix as f32;
          mat.ix_length_flipx_flipy[1] = data.length as f32;
          mat.ix_length_flipx_flipy[2] = if data.flip_x { -1.0 } else { 1.0 };
          mat.ix_length_flipx_flipy[3] = if data.flip_y { -1.0 } else { 1.0 };
          mat.xoff_yoff_xrep_yrep[0] =
              (mat.xoff_yoff_xrep_yrep[0] + data.scroll.x * time_factor).rem_euclid(1.0);
          mat.xoff_yoff_xrep_yrep[1] =
              (mat.xoff_yoff_xrep_yrep[1] + data.scroll.y * time_factor).rem_euclid(1.0);
          match manager.growth {
              AnimationGrowth::Repeat => {
                  let mesh_size = uvec2_bound(&manager.points);
                  let image_size = current_node.sprite.size;
                  mat.xoff_yoff_xrep_yrep[2] = mesh_size.x as f32 / image_size.x as f32;
                  mat.xoff_yoff_xrep_yrep[3] = mesh_size.y as f32 / image_size.y as f32;
              }
              AnimationGrowth::Stretch => {
                  mat.xoff_yoff_xrep_yrep[2] = 1.0;
                  mat.xoff_yoff_xrep_yrep[3] = 1.0;
              }
          }
          // Then progress the animation (so in case it swaps it'll be correct by next frame)
          let current_node = current_node.clone();
          index.time += time_factor;
          if index.time > data.spf {
              index.ix += 1;
              index.time -= data.spf;
          }
          if index.ix >= data.length {
              index.ix = 0;
              if current_node.next.is_some() {
                  manager.set_key(current_node.next.unwrap(), &mut commands);
              }
          }
      }
  }

pub(super) fn register_manager(app: &mut App) {
    
    app.add_systems(
        PreUpdate,
        (
            stabilize_multi_animations,update_animation_bodies
        )
            .in_set(AnimationSet),
    );
    
    app.add_systems(Update, play_animations.in_set(AnimationSet));

    app.register_type::<AnimationBodyMarker>();
    app.register_type::<AnimationMap>();
    app.register_type::<AnimationManager>();
    app.register_type::<AnimationIndex>();
    app.register_type::<AnimationData>();
    app.register_type::<AnimationMaterial>();
    app.register_type::<MultiAnimationManager>();
}
