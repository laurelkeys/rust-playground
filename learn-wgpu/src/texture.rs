use anyhow::*;
use image::GenericImageView;
use std::path::Path;

//
// Texture
//

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

/// Assuming texture images use the sRGB color space, as they very often do,
/// this represents whether its colors are stored linearly or non-linearly.
// @Note: an "encoded" sRGB color has the sRGB OETF applied.
pub enum TextureIsSrgb {
    Linear,
    Encoded,
}

impl Texture {
    //
    // Depth textures.
    //

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: Option<&str>,
    ) -> Self {
        let size =
            wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..wgpu::SamplerDescriptor::default()
        });

        Self { texture, view, sampler }
    }

    //
    // Image textures.
    //

    #[allow(dead_code)]
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: Option<&str>,
        srgb: TextureIsSrgb,
    ) -> Result<Self> {
        let image = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &image, label, srgb)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
        srgb: TextureIsSrgb,
    ) -> Result<Self> {
        let (width, height) = image.dimensions();
        anyhow::ensure!(width > 0 && height > 0);

        let size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: match srgb {
                TextureIsSrgb::Linear => wgpu::TextureFormat::Rgba8Unorm,
                TextureIsSrgb::Encoded => wgpu::TextureFormat::Rgba8UnormSrgb,
            },
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.to_rgba8(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width), // RGBA => 4
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..wgpu::SamplerDescriptor::default()
        });

        Ok(Self { texture, view, sampler })
    }

    pub fn load<P>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: P,
        srgb: TextureIsSrgb,
    ) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(&path)?;
        let label = path.as_ref().to_str();
        Self::from_image(device, queue, &image, label, srgb)
    }
}
