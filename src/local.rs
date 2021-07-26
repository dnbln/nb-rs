pub trait LocalNekosBestCategory {
    const CATEGORY: crate::Category;
    const MIN: usize;
    const MAX: usize;
    const WITH_PADDING: usize;
    const FORMAT: &'static str;

    fn get_random(&self, random: usize) -> String {
        format!(
            "{}/{}/{:0width$}{}",
            super::BASE_URL,
            Self::CATEGORY,
            random % (Self::MAX - Self::MIN + 1) + Self::MIN,
            Self::FORMAT,
            width = Self::WITH_PADDING
        )
    }

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