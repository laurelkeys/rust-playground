use anyhow::*;
use cgmath::{InnerSpace, Vector2, Vector3, Zero};
use std::{ops::Range, path::Path};
use wgpu::util::DeviceExt;

use crate::texture;

//
// Vertex
//

pub trait Vertex {
    /// Returns a descriptor of how the vertex buffer is interpreted.
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    texcoord: [f32; 2],
    normal: [f32; 3],
    tangent: [f32; 3],
    bitangent: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position: [f32; 3],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                // texcoord: [f32; 2],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                // normal: [f32; 3],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
                // tangent: [f32; 3],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                // bitangent: [f32; 3],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
            ],
        }
    }
}

//
// Mesh
//

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub indices_count: u32,
    pub material_index: usize,
}

//
// Material
//

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

//
// Model, DrawModel
//

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn load<P>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: P,
    ) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let (obj_models, obj_materials) = tobj::load_obj(
            path.as_ref(),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..tobj::LoadOptions::default()
            },
        )
        .expect("Failed to load .obj model file");

        let obj_materials = obj_materials.expect("Failed to load .mtl model file");

        // @Robustness: we assume that the textures are stored with the .obj file.
        let folder = path.as_ref().parent().context("Path to model has no parent")?;

        let materials = obj_materials
            .into_iter()
            .map(|material| {
                let diffuse_texture =
                    texture::Texture::load(device, queue, folder.join(material.diffuse_texture))?;
                let normal_texture =
                    texture::Texture::load(device, queue, folder.join(material.normal_texture))?;

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("bind_group"),
                    layout,
                    entries: &[
                        // Diffuse texture:
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                        },
                        // Normal map texture:
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                        },
                    ],
                });

                Ok(Material { name: material.name, diffuse_texture, normal_texture, bind_group })
            })
            .collect::<Result<Vec<Material>>>()?;

        let meshes = obj_models
            .into_iter()
            .map(|model| {
                #[derive(Clone, Copy)]
                struct TangentBitangent(Vector3<f32>, Vector3<f32>);
                struct PositionTexcoordNormal(Vector3<f32>, Vector2<f32>, Vector3<f32>);

                let tobj::Mesh { positions, normals, texcoords, indices, .. } = &model.mesh;

                assert!(indices.len() % 3 == 0);
                assert!(positions.len() % 3 == 0);
                let vertices_count = positions.len() / 3;

                let vertices_without_tangent_and_bitangent = (0..vertices_count)
                    .map(|i| {
                        PositionTexcoordNormal(
                            Vector3 {
                                x: positions[i * 3],
                                y: positions[i * 3 + 1],
                                z: positions[i * 3 + 2],
                            },
                            Vector2 { x: texcoords[i * 2], y: texcoords[i * 2 + 1] },
                            Vector3 {
                                x: normals[i * 3],
                                y: normals[i * 3 + 1],
                                z: normals[i * 3 + 2],
                            },
                        )
                    })
                    .collect::<Vec<_>>();

                let vertices_tangent_and_bitangent = indices.chunks_exact(3).into_iter().fold(
                    vec![TangentBitangent(Vector3::zero(), Vector3::zero()); vertices_count],
                    |mut vertices_tangent_and_bitangent, is| {
                        let (i0, i1, i2) = (is[0] as usize, is[1] as usize, is[2] as usize);

                        let (
                            PositionTexcoordNormal(position0, texcoord0, _),
                            PositionTexcoordNormal(position1, texcoord1, _),
                            PositionTexcoordNormal(position2, texcoord2, _),
                        ) = (
                            &vertices_without_tangent_and_bitangent[i0],
                            &vertices_without_tangent_and_bitangent[i1],
                            &vertices_without_tangent_and_bitangent[i2],
                        );

                        let (d_xyz01, d_xyz02) = (position1 - position0, position2 - position0);
                        let (d_uv01, d_uv02) = (texcoord1 - texcoord0, texcoord2 - texcoord0);

                        // Compute the tangent and bitangent vectors T and B, such that:
                        //   d_xyz01 == d_uv01.x * T + d_uv01.y * B
                        //   d_xyz02 == d_uv02.x * T + d_uv01.y * B
                        let r = 1.0 / (d_uv01.x * d_uv02.y - d_uv01.y * d_uv02.x);
                        let tangent = r * (d_xyz01 * d_uv02.y - d_xyz02 * d_uv01.y);
                        let bitangent = r * (d_xyz02 * d_uv01.x - d_xyz01 * d_uv02.x);

                        // @Note: we use the same values for each vertex in the triangle.
                        vertices_tangent_and_bitangent[i0].0 += tangent;
                        vertices_tangent_and_bitangent[i0].1 += bitangent;
                        vertices_tangent_and_bitangent[i1].0 += tangent;
                        vertices_tangent_and_bitangent[i1].1 += bitangent;
                        vertices_tangent_and_bitangent[i2].0 += tangent;
                        vertices_tangent_and_bitangent[i2].1 += bitangent;

                        vertices_tangent_and_bitangent
                    },
                );

                let vertices = vertices_without_tangent_and_bitangent
                    .into_iter()
                    .zip(vertices_tangent_and_bitangent.into_iter())
                    .map(
                        |(
                            PositionTexcoordNormal(position, texcoord, normal),
                            TangentBitangent(tangent, bitangent),
                        )| {
                            cgmath::assert_abs_diff_eq!(normal.magnitude2(), 1.0, epsilon = 0.001);

                            ModelVertex {
                                position: position.into(),
                                texcoord: texcoord.into(),
                                normal: normal.into(),
                                tangent: tangent.normalize().into(),
                                bitangent: bitangent.normalize().into(),
                            }
                        },
                    )
                    .collect::<Vec<_>>();

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} vertex_buffer", path.as_ref())),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} index_buffer", path.as_ref())),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                Ok(Mesh {
                    name: model.name,
                    vertex_buffer,
                    index_buffer,
                    indices_count: indices.len() as u32,
                    material_index: model.mesh.material_id.unwrap_or(0),
                })
            })
            .collect::<Result<Vec<Mesh>>>()?;

        Ok(Self { meshes, materials })
    }
}

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );

    fn draw_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );
}

impl<'renderpass, 'a> DrawModel<'a> for wgpu::RenderPass<'renderpass>
where
    'a: 'renderpass,
{
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, camera_bind_group, light_bind_group, 0..1);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);

        self.draw_indexed(0..mesh.indices_count, 0, instances);
    }

    fn draw_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.draw_model_instanced(model, camera_bind_group, light_bind_group, 0..1);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material_index];
            self.draw_mesh_instanced(
                mesh,
                material,
                camera_bind_group,
                light_bind_group,
                instances.clone(),
            );
        }
    }
}

//
// DrawLight
//

pub trait DrawLight<'a> {
    fn draw_light_mesh(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );

    fn draw_light_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_light_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );
}

impl<'renderpass, 'a> DrawLight<'a> for wgpu::RenderPass<'renderpass>
where
    'a: 'renderpass,
{
    fn draw_light_mesh(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.draw_light_mesh_instanced(mesh, camera_bind_group, light_bind_group, 0..1);
    }

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, light_bind_group, &[]);

        self.draw_indexed(0..mesh.indices_count, 0, instances);
    }

    fn draw_light_model(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        self.draw_light_model_instanced(model, camera_bind_group, light_bind_group, 0..1);
    }
    fn draw_light_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        for mesh in &model.meshes {
            self.draw_light_mesh_instanced(
                mesh,
                camera_bind_group,
                light_bind_group,
                instances.clone(),
            );
        }
    }
}
