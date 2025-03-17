use std::{convert::TryFrom, str::FromStr};

macro_rules! categories {
    ($(
        $(#[$at:meta])* $(ref $(#[$ref_at:meta])*)?
        $cat_name:ident => $url_name:literal,
    )*) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(into = "String", try_from = "String")]
        #[non_exhaustive]
        pub enum Category {
            $(
                $(#[$at])*
                $cat_name,
            )*
        }

        impl FromStr for Category {
            type Err = NoSuchVariant;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_url_name(s).ok_or(NoSuchVariant)
            }
        }

        impl Category {
            pub const fn to_url_name(self) -> &'static str {
                match self {
                    $(
                        $(#[$ref_at])*
                        Category::$cat_name => $url_name,
                    )*
                }
            }

            pub fn from_url_name(name: &str) -> Option<Self> {
                match name {
                    $(
                        $url_name => Some($(#[$ref_at])* {Category::$cat_name}),
                    )*
                    _ => None,
                }
            }

            pub const ALL_VARIANTS: &[Category] = &[
                $(
                    $(#[$ref_at])*
                    Category::$cat_name,
                )*
            ];
        }
    };
}

categories! {
    Angry => "angry",
    Baka => "baka",
    Bite => "bite",
    Blush => "blush",
    Bored => "bored",
    Cry => "cry",
    Cuddle => "cuddle",
    Dance => "dance",
    Facepalm => "facepalm",
    Feed => "feed",
    Handhold => "handhold",
    Handshake => "handshake",
    Happy => "happy",
    Highfive => "highfive",
    Hug => "hug",
    Husbando => "husbando",
    Kick => "kick",
    Kiss => "kiss",
    Kitsune => "kitsune",
    Laugh => "laugh",
    Lurk => "lurk",
    Neko => "neko",
    Nod => "nod",
    Nom => "nom",
    Nope => "nope",
    Pat => "pat",
    Peck => "peck",
    Poke => "poke",
    Pout => "pout",
    Punch => "punch",
    Run => "run",
    Shoot => "shoot",
    Shrug => "shrug",
    Slap => "slap",
    Sleep => "sleep",
    Smile => "smile",
    Smug => "smug",
    Stare => "stare",
    Think => "think",
    ThumbsUp => "thumbsup",
    Tickle => "tickle",
    Waifu => "waifu",
    Wave => "wave",
    Wink => "wink",
    Yawn => "yawn",
    Yeet => "yeet",
}

#[derive(thiserror::Error, Debug)]
#[error("no such variant")]
pub struct NoSuchVariant;

impl TryFrom<String> for Category {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl Into<String> for Category {
    fn into(self) -> String {
        self.to_url_name().to_owned()
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_url_name().fmt(f)
    }
}
