//! Allows getting image urls locally, without
//! making any request to the API, using only data
//! available from the `GET /endpoints` endpoint,
//! and implementing [`LocalNekosBestCategory`] with
//! the corresponding data on the generated structs.
//! Also see the build script.

/// A trait to generate image urls locally
pub trait LocalNekosBestCategory {
    /// The category
    const CATEGORY: crate::Category;
    /// The lower bound of the range of available images
    const MIN: usize;
    /// The upper bound of the range of available images
    const MAX: usize;
    /// The length of file names with 0 left-padding.
    const WITH_PADDING: usize;
    /// The format of the files (and extension)
    const FORMAT: &'static str;

    // Gets a random image url given a random number.
    fn get_random(&self, random: usize) -> String {
        format!(
            "{}/{}/{:0width$}.{}",
            super::BASE_URL,
            Self::CATEGORY,
            random % (Self::MAX - Self::MIN + 1) + Self::MIN,
            Self::FORMAT,
            width = Self::WITH_PADDING
        )
    }

    // Gets a random image by invoking [`get_random`] with a number from the thread_rng.
    fn get(&self) -> String {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        self.get_random(rng.gen())
    }
}

mod implementation {
    include!(concat!(env!("OUT_DIR"), "/local_implementation.rs"));
}

pub use implementation::*;