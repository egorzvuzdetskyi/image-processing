use image::imageops::FilterType;
use std::fmt;

#[derive(Clone, Copy)]
pub enum InterpolationMethod {
    Nearest,
    Triangle,
    Cubic,
    Gaussian,
    Lanczos3,
}

impl InterpolationMethod {
    const METHODS: &'static [&'static str] =
        &["nearest", "triangle", "cubic", "gaussian", "lanczos3"];

    pub fn as_str(&self) -> &'static str {
        Self::METHODS[*self as usize]
    }

    pub fn matches_str(s: &str) -> bool {
        Self::METHODS.contains(&s)
    }

    pub fn to_external_image_methods(&self) -> FilterType {
        match &self {
            Self::Nearest => FilterType::Nearest,
            Self::Triangle => FilterType::Triangle,
            Self::Cubic => FilterType::CatmullRom,
            Self::Gaussian => FilterType::Gaussian,
            Self::Lanczos3 => FilterType::Lanczos3,
        }
    }

    pub fn from_str(method: &str) -> Option<Self> {
        match Self::METHODS.iter().position(|&m| m == method) {
            Some(0) => Some(Self::Nearest),
            Some(1) => Some(Self::Triangle),
            Some(2) => Some(Self::Cubic),
            Some(3) => Some(Self::Gaussian),
            Some(4) => Some(Self::Lanczos3),
            _ => None,
        }
    }
}

impl fmt::Display for InterpolationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::METHODS[*self as usize])
    }
}
