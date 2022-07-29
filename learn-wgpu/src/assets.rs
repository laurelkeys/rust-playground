use anyhow::Context;
use cgmath::{InnerSpace, Vector2, Vector3, Zero};
use wgpu::util::DeviceExt;

use crate::{assets, model, texture};

// fn load_string(file_name: &str) -> anyhow::Result<String> {
//     let path = std::path::Path::new(env!("OUT_DIR")).join("assets").join(file_name);
//     Ok(std::fs::read_to_string(path)?)
// }

// fn load_bytes(file_name: &str) -> anyhow::Result<Vec<u8>> {
//     let path = std::path::Path::new(env!("OUT_DIR")).join("assets").join(file_name);
//     Ok(std::fs::read(path)?)
// }

pub fn load_texture(
    file_name: &str,
    is_srgb: texture::TextureIsSrgb,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture> {
    let path = std::path::Path::new(env!("OUT_DIR")).join("assets").join(file_name);
    let bytes = std::fs::read(path)?;
    texture::Texture::from_bytes(device, queue, &bytes, Some(file_name), is_srgb)
}

pub fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<model::Model> {
    let path = std::path::Path::new(env!("OUT_DIR")).join("assets").join(file_name);
    // @Robustness: we assume that the textures are stored with the .obj file.
    let parent = path.parent().context("Path to model has no parent")?;

    let (obj_models, obj_materials) = tobj::load_obj(
        &path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..tobj::LoadOptions::default()
        },
    )
    .expect("Failed to load .obj model file");

    let materials = obj_materials
        .expect("Failed to load .mtl model file")
        .into_iter()
        .map(|material| {
            let diffuse_texture = assets::load_texture(
                parent.join(material.diffuse_texture).to_str().unwrap(),
                texture::TextureIsSrgb::Encoded,
                device,
                queue,
            )?;

            let normal_texture = assets::load_texture(
                parent.join(material.normal_texture).to_str().unwrap(),
                texture::TextureIsSrgb::Linear,
                device,
                queue,
            )?;

            Ok(model::Material::new(
                device,
                layout,
                &material.name,
                diffuse_texture,
                normal_texture,
            ))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let meshes = obj_models
        .into_iter()
        .map(|model| {
            let vertices = model_vertices_from_obj_model(&model);

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} vertex_buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} index_buffer", file_name)),
                contents: bytemuck::cast_slice(&model.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            Ok(model::Mesh {
                name: model.name,
                vertex_buffer,
                index_buffer,
                indices_count: model.mesh.indices.len() as u32,
                material_index: model.mesh.material_id.unwrap_or(0),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(model::Model { meshes, materials })
}

fn model_vertices_from_obj_model(model: &tobj::Model) -> Vec<model::ModelVertex> {
    let tobj::Mesh { positions, normals, texcoords, indices, .. } = &model.mesh;
    assert_eq!(indices.len() % 3, 0);
    assert_eq!(positions.len() % 3, 0);
    let vertices_count = positions.len() / 3;

    let vertices_position_texcoord_normal = (0..vertices_count)
        .map(|i| {
            (
                Vector3::from([positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]]),
                Vector2::from([texcoords[i * 2], texcoords[i * 2 + 1]]),
                Vector3::from([normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]]),
            )
        })
        .collect::<Vec<_>>();

    let vertices_tangent_bitangent = indices
        .chunks_exact(3)
        .fold(
            vec![(Vector3::zero(), Vector3::zero(), 0); vertices_count],
            |mut vertices_tangent_bitangent_and_counter, is| {
                let (i0, i1, i2) = (is[0] as usize, is[1] as usize, is[2] as usize);

                let (position0, texcoord0, _) = &vertices_position_texcoord_normal[i0];
                let (position1, texcoord1, _) = &vertices_position_texcoord_normal[i1];
                let (position2, texcoord2, _) = &vertices_position_texcoord_normal[i2];

                let (d_xyz01, d_xyz02) = (position1 - position0, position2 - position0);
                let (d_uv01, d_uv02) = (texcoord1 - texcoord0, texcoord2 - texcoord0);

                // Compute the tangent and bitangent vectors T and B, such that:
                //   d_xyz01 == d_uv01.x * T + d_uv01.y * B
                //   d_xyz02 == d_uv02.x * T + d_uv02.y * B
                let rcp = 1.0 / (d_uv01.x * d_uv02.y - d_uv01.y * d_uv02.x);
                let tangent = rcp * (d_uv02.y * d_xyz01 - d_uv01.y * d_xyz02);
                let bitangent = rcp * (d_uv01.x * d_xyz02 - d_uv02.x * d_xyz01); // @@

                // @Note: we use the same values for each vertex in the triangle.
                vertices_tangent_bitangent_and_counter[i0].0 += tangent;
                vertices_tangent_bitangent_and_counter[i0].1 += bitangent;
                vertices_tangent_bitangent_and_counter[i0].2 += 1;

                vertices_tangent_bitangent_and_counter[i1].0 += tangent;
                vertices_tangent_bitangent_and_counter[i1].1 += bitangent;
                vertices_tangent_bitangent_and_counter[i1].2 += 1;

                vertices_tangent_bitangent_and_counter[i2].0 += tangent;
                vertices_tangent_bitangent_and_counter[i2].1 += bitangent;
                vertices_tangent_bitangent_and_counter[i2].2 += 1;

                vertices_tangent_bitangent_and_counter
            },
        )
        .into_iter()
        .map(|(tangent, bitangent, counter)| {
            assert_ne!(counter, 0);
            let rcp_counter = 1.0 / counter as f32;
            (tangent * rcp_counter, bitangent * rcp_counter)
        })
        .collect::<Vec<_>>();

    vertices_position_texcoord_normal
        .into_iter()
        .zip(vertices_tangent_bitangent)
        .map(|((position, texcoord, normal), (tangent, bitangent))| {
            cgmath::assert_abs_diff_eq!(normal.magnitude2(), 1.0, epsilon = 0.001);

            model::ModelVertex {
                position: position.into(),
                texcoord: texcoord.into(),
                normal: normal.into(),
                tangent: tangent.normalize().into(),
                bitangent: bitangent.normalize().into(),
            }
        })
        .collect::<Vec<_>>()
}
