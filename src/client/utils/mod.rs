use bevy::{prelude::*, render::mesh::{PrimitiveTopology, Indices}};


pub fn create_mesh(
  meshes: &mut ResMut<Assets<Mesh>>,
  positions: Vec<[f32; 3]>,
  normals: Vec<[f32; 3]>,
  uvs: Vec<[f32; 2]>,
  indices: Vec<u32>,
) -> Handle<Mesh> {
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
  mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
  mesh.set_indices(Some(Indices::U32(indices)));
  meshes.add(mesh)
}
