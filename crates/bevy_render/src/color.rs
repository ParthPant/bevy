use super::texture::Texture;
use crate::shader::ShaderDefSuffixProvider;
use bevy_asset::Handle;
use bevy_core::bytes::GetBytes;
use glam::Vec4;
use std::ops::{Add, AddAssign};
use zerocopy::AsBytes;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, AsBytes)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Add<Vec4> for Color {
    type Output = Color;
    fn add(self, rhs: Vec4) -> Self::Output {
        Color {
            r: self.r + rhs.x(),
            g: self.g + rhs.y(),
            b: self.b + rhs.z(),
            a: self.a + rhs.w(),
        }
    }
}

impl From<Vec4> for Color {
    fn from(vec4: Vec4) -> Self {
        Color {
            r: vec4.x(),
            g: vec4.y(),
            b: vec4.z(),
            a: vec4.w(),
        }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl GetBytes for Color {
    fn get_bytes(&self) -> Vec<u8> {
        self.as_bytes().iter().map(|v| *v).collect::<Vec<u8>>()
    }
    fn get_bytes_ref(&self) -> Option<&[u8]> {
        Some(self.as_bytes())
    }
}

pub enum ColorSource {
    Color(Color),
    Texture(Handle<Texture>),
}

impl From<Vec4> for ColorSource {
    fn from(vec4: Vec4) -> Self {
        ColorSource::Color(vec4.into())
    }
}

impl From<Color> for ColorSource {
    fn from(color: Color) -> Self {
        ColorSource::Color(color)
    }
}

impl From<Handle<Texture>> for ColorSource {
    fn from(texture: Handle<Texture>) -> Self {
        ColorSource::Texture(texture)
    }
}

impl ShaderDefSuffixProvider for ColorSource {
    fn get_shader_def(&self) -> Option<&'static str> {
        match *self {
            ColorSource::Color(_) => Some("_COLOR"),
            ColorSource::Texture(_) => Some("_TEXTURE"),
        }
    }
}

impl GetBytes for ColorSource {
    fn get_bytes(&self) -> Vec<u8> {
        match *self {
            ColorSource::Color(ref color) => color.get_bytes(),
            ColorSource::Texture(_) => Vec::new(), // Texture is not a uniform
        }
    }
    fn get_bytes_ref(&self) -> Option<&[u8]> {
        match *self {
            ColorSource::Color(ref color) => color.get_bytes_ref(),
            ColorSource::Texture(ref texture) => texture.get_bytes_ref(), // Texture is not a uniform
        }
    }
}

pub mod colors {
    use super::Color;

    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
}