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

pub trait StrEnum {
    // Associated constant for string values
    const VALUES: &'static [&'static str];

    // Convert enum to string
    fn as_str(&self) -> &'static str
    where
        Self: Sized + Copy + Into<usize>,
    {
        Self::VALUES[(*self).into()]
    }

    // Check if string matches any enum value
    fn matches_str(s: &str) -> bool
    where
        Self: Sized,
    {
        Self::VALUES.contains(&s)
    }

    // Convert string to enum
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

impl fmt::Display for InterpolationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::VALUES[*self as usize])
    }
}
