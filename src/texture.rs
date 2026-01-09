use std::{cell::RefCell, rc::Rc};

use miniquad::{RenderingBackend, TextureId, TextureParams};

pub struct TextureContext {
    backend: Rc<RefCell<Box<dyn RenderingBackend>>>,
}

impl TextureContext {
    pub(crate) fn new(backend: Rc<RefCell<Box<dyn RenderingBackend>>>) -> Self {
        Self { backend }
    }

    pub fn register_texture_rgba8(&self, width: u16, height: u16, buffer: &[u8]) -> TextureId {
        let mut backend_mut = self.backend.borrow_mut();

        assert_eq!(
            width as usize * height as usize * 4,
            buffer.len(),
            "Expected {} * {} * 4 = {} bytes, got {} bytes in buffer",
            width,
            height,
            width * height * 4,
            buffer.len()
        );

        backend_mut.new_texture_from_data_and_format(
            buffer,
            TextureParams {
                width: width as u32,
                height: height as u32,
                format: miniquad::TextureFormat::RGBA8,
                ..Default::default()
            },
        )
    }

    pub fn register_texture_rgb8(&self, width: u16, height: u16, buffer: &[u8]) -> TextureId {
        let mut backend_mut = self.backend.borrow_mut();

        assert_eq!(
            width as usize * height as usize * 3,
            buffer.len(),
            "Expected {} * {} * 3 = {} bytes, got {} bytes in buffer",
            width,
            height,
            width * height * 3,
            buffer.len()
        );

        backend_mut.new_texture_from_data_and_format(
            buffer,
            TextureParams {
                width: width as u32,
                height: height as u32,
                format: miniquad::TextureFormat::RGB8,
                ..Default::default()
            },
        )
    }
}
