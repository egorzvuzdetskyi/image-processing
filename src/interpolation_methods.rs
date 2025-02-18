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

pub trait StrEnum: Sized + Copy + Into<usize> {
    const VALUES: &'static [&'static str];

    fn as_str(&self) -> &'static str
    where
        Self: Sized + Copy + Into<usize>,
    {
        Self::VALUES[(*self).into()]
    }

    fn matches_str(s: &str) -> bool
    where
        Self: Sized,
    {
        Self::VALUES.contains(&s)
    }

    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
}

impl StrEnum for InterpolationMethod {
    const VALUES: &'static [&'static str] =
        &["nearest", "triangle", "cubic", "gaussian", "lanczos3"];

    fn from_str(s: &str) -> Option<Self> {
        match Self::VALUES.iter().position(|&m| m == s) {
            Some(0) => Some(Self::Nearest),
            Some(1) => Some(Self::Triangle),
            Some(2) => Some(Self::Cubic),
            Some(3) => Some(Self::Gaussian),
            Some(4) => Some(Self::Lanczos3),
            _ => None,
        }
    }
}

impl InterpolationMethod {
    const VALUES: &'static [&'static str] =
        &["nearest", "triangle", "cubic", "gaussian", "lanczos3"];

    pub fn as_str(&self) -> &'static str {
        <Self as StrEnum>::as_str(self) // Explicitly call the trait method
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
}

impl Into<usize> for InterpolationMethod {
    fn into(self) -> usize {
        match self {
            Self::Nearest => 0,
            Self::Triangle => 1,
            Self::Cubic => 2,
            Self::Gaussian => 3,
            Self::Lanczos3 => 4,
        }
    }
}

impl fmt::Display for InterpolationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::VALUES[*self as usize])
    }
}
