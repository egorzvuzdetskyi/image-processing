use std::fmt;

#[derive(Copy, Clone)]
pub enum Commands {
    Resize,
    Upscale,
    Downscale,
}

impl Commands {
    const COMANDS: &'static [&'static str] = &["resize", "upscale", "downscale"];

    pub fn as_str(&self) -> &'static str {
        Self::COMANDS[*self as usize]
    }

    pub fn matches_str(s: &str) -> bool {
        Self::COMANDS.contains(&s)
    }

    pub fn from_str(command: &str) -> Option<Self> {
        match Self::COMANDS.iter().position(|&m| m == command) {
            Some(0) => Some(Self::Resize),
            Some(1) => Some(Self::Upscale),
            Some(2) => Some(Self::Downscale),
            _ => None,
        }
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::COMANDS[*self as usize])
    }
}
