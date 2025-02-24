pub trait StrEnum: Sized + Copy + Into<usize> {
    const VALUES: &'static [&'static str];

    fn as_str(&self) -> &'static str {
        Self::VALUES[(*self).into()]
    }

    fn matches_str(s: &str) -> bool {
        Self::VALUES.contains(&s)
    }

    fn from_str(s: &str) -> Option<Self>;
}
