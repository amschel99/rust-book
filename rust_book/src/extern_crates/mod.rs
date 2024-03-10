//! Vinyl Generator
//! A rust crate to create beautiful vinyl videos.
pub mod vinyl_image {
    pub fn generate() {}
    pub fn resize() {}
}

pub mod video {
    pub fn overlay() {}
    pub fn change_resolution() {}
}

/// # Examples
///
/// ```
/// use crate::extern_crates::add::add_num;
///  assert_eq(3, add_num(2));
///
/// ```
pub mod add {
    pub fn add_num(x: i32) -> i32 {
        x + 1
    }
}
