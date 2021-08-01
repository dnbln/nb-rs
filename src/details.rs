//! Details of an [`crate::Category::Nekos`] image.
//! Refer to [`NekosDetails`].

/// In the case of [`Category::Nekos`], the API
/// also returns the source url, the name and a
/// link to the artist that made it.
#[derive(serde::Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NekosDetails {
    pub artist_href: String,
    pub artist_name: String,
    pub source_url: String,
}