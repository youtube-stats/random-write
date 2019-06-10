use std::ops::Range;

pub const POSTGRESQL_URL: &'static str = "postgresql://admin@localhost:5432/youtube";
pub const CACHE_SIZE: usize = 1000;
pub const DRAIN_RANGE: Range<usize> = 0..CACHE_SIZE;