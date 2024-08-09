use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, RenderPipelineDescriptor,
    ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey};

use bevy::{
      render::{
          mesh::{Indices, PrimitiveTopology},
          render_asset::RenderAssetUsages,
      },
      sprite::Mesh2dHandle,
  };
  


pub const SPRITE_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(8267429772218888889);

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct AnimationMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
    // The below need to be packed for wasm where stuff has to be 16-byte aligned
    #[uniform(3)]
    pub ix_length_flipx_flipy: Vec4, // NOTE: 1.0 = don't flip, -1.0 = flip
    #[uniform(4)]
    pub xoff_yoff_xrep_yrep: Vec4,
    #[uniform(5)]
    pub rgba: Vec4,
}
impl AnimationMaterial {
    pub fn from_handle(
        handle: Handle<Image>,
        length: u32,
        repetitions: Vec2,
        color: Color,
    ) -> Self {
        let srgba_thing = color.to_srgba();
        Self {
            texture: handle,
            ix_length_flipx_flipy: Vec4::new(0.0, length as f32, 1.0, 1.0),
            xoff_yoff_xrep_yrep: Vec4::new(0.0, 0.0, repetitions.x, repetitions.y),
            rgba: Vec4::new(
                srgba_thing.red,
                srgba_thing.green,
                srgba_thing.blue,
                srgba_thing.alpha,
            ),
        }
    }
}

impl Material2d for AnimationMaterial {
      fn fragment_shader() -> ShaderRef {
          "shaders/animation_mat.wgsl".into()
      }
}


pub const BLEND_ADD: BlendState = BlendState {
    color: BlendComponent {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    },
    alpha: BlendComponent {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    },
};

pub fn points_to_mesh(points: &[Vec2], meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle {
      let mut points_vec: Vec<f32> = vec![];
      let mut top_left = Vec2::new(f32::MAX, f32::MAX);
      let mut bot_right = Vec2::new(f32::MIN, f32::MIN);
      for point in points.iter() {
          points_vec.push(point.x);
          points_vec.push(point.y);
          top_left.x = top_left.x.min(point.x);
          top_left.y = top_left.y.min(point.y);
          bot_right.x = bot_right.x.max(point.x);
          bot_right.y = bot_right.y.max(point.y);
      }
      let verts: Vec<u32> = earcutr::earcut(&points_vec, &[], 2)
          .unwrap()
          .into_iter()
          .map(|val| val as u32)
          .collect();
      let mut triangle = Mesh::new(
          PrimitiveTopology::TriangleList,
          RenderAssetUsages::RENDER_WORLD,
      );
      let mut positions: Vec<[f32; 3]> = vec![];
      let mut normals: Vec<[f32; 3]> = vec![];
      let mut uvs: Vec<[f32; 2]> = vec![];
      for p in points.iter() {
          positions.push([p.x, p.y, 0.0]);
          normals.push([0.0, 0.0, 1.0]);
          let uv_x = (p.x - top_left.x) / (bot_right.x - top_left.x);
          // I'm only 80% confident this should be 1.0 -
          let uv_y = 1.0 - (p.y - top_left.y) / (bot_right.y - top_left.y);
          uvs.push([uv_x, uv_y]);
      }
      triangle.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
      triangle.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
      triangle.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
      triangle.insert_indices(Indices::U32(verts));
      meshes.add(triangle).into()
  }