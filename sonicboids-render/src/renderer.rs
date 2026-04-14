//! Rendering infrastructure for sonicboids

use std::cell::{Cell, RefCell};

use nannou::Frame;
use nannou::draw;
use nannou::wgpu;

/// Ping-pong texture feedback renderer.
///
/// Each frame, the previous frame's output is drawn back as a faded history
/// background, then current content is drawn on top. The result is rendered
/// into an off-screen texture via nannou's `draw::Renderer`, then displayed
/// to the window. This gives O(1) GPU cost for history regardless of its age.
pub struct FeedbackRenderer {
    textures: [wgpu::Texture; 2],

    // The renderer is wrapped in a RefCell because Nannou's view function is
    // immutable. RefCell allows a mutable borrow, and safe because submit()
    // is only called once per frame.
    renderer: RefCell<draw::Renderer>,

    // The read_idx is wrapped in Cell for the same reason as above. Since usize
    // is `Copy`, no borrow bookkeeping is needed.
    read_idx: Cell<usize>,

    // The size of the texture
    size: [u32; 2],
}

impl FeedbackRenderer {
    pub fn new(device: &wgpu::Device, size: [u32; 2]) -> Self {
        let format = Frame::TEXTURE_FORMAT;
        let usage = wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT;

        let build_tex = || {
            wgpu::TextureBuilder::new()
                .size(size)
                .usage(usage)
                .format(format)
                .build(device)
        };

        let textures = [build_tex(), build_tex()];

        let renderer = draw::RendererBuilder::new().build(
            device, size, 1.0, // scale_factor
            1,   // sample_count (no MSAA)
            format,
        );

        Self {
            textures,
            renderer: RefCell::new(renderer),
            read_idx: Cell::new(0),
            size,
        }
    }

    pub fn size(&self) -> [u32; 2] {
        self.size
    }

    /// The texture containing the previous frame's accumulated output.
    pub fn read_texture(&self) -> &wgpu::Texture {
        &self.textures[self.read_idx.get()]
    }

    /// Render `draw` into the write texture, then flip read <-> write.
    ///
    /// Call this after all draw commands for the current frame have been issued.
    pub fn submit(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        draw: &draw::Draw,
    ) {
        // The Write texture is the other texture from Read
        let write_idx = 1 - self.read_idx.get();

        // Render to the Write texture
        self.renderer.borrow_mut().render_to_texture(
            device,
            encoder,
            draw,
            &self.textures[write_idx],
        );

        // Set the Read texture to the Write texture for next frame
        self.read_idx.set(write_idx);
    }
}
