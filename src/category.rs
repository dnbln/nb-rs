use std::{convert::TryFrom, str::FromStr};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(into = "String", try_from = "String")]
pub enum Category {
    Baka,
    Cry,
    Cuddle,
    Dance,
    Feed,
    Hug,
    Kiss,
    Laugh,
    Nekos,
    Pat,
    Poke,
    Slap,
    Smile,
    Smug,
    Tickle,
    Wave,

    Bite,
    Blush,
    Bored,
    Facepalm,
    Happy,
    Highfive,
    Pout,
    Shrug,
    Sleep,
    Stare,
    Think,
    ThumbsUp,
    Wink,
}

#[derive(thiserror::Error, Debug)]
#[error("no such variant")]
pub struct NoSuchVariant;

impl FromStr for Category {
    type Err = NoSuchVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = match s {
            "baka" => Category::Baka,
            "cry" => Category::Cry,
            "cuddle" => Category::Cuddle,
            "dance" => Category::Dance,
            "feed" => Category::Feed,
            "hug" => Category::Hug,
            "kiss" => Category::Kiss,
            "laugh" => Category::Laugh,
            "nekos" => Category::Nekos,
            "pat" => Category::Pat,
            "poke" => Category::Poke,
            "slap" => Category::Slap,
            "smile" => Category::Smile,
            "smug" => Category::Smug,
            "tickle" => Category::Tickle,
            "wave" => Category::Wave,

            "bite" => Category::Bite,
            "blush" => Category::Blush,
            "bored" => Category::Bored,
            "facepalm" => Category::Facepalm,
            "happy" => Category::Happy,
            "highfive" => Category::Highfive,
            "pout" => Category::Pout,
            "shrug" => Category::Shrug,
            "sleep" => Category::Sleep,
            "stare" => Category::Stare,
            "think" => Category::Think,
            "tumbsup" => Category::ThumbsUp,
            "wink" => Category::Wink,
            _ => return Err(NoSuchVariant),
        };

        Ok(c)
    }
}

impl TryFrom<String> for Category {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl Into<String> for Category {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Category {
    pub const fn to_url_path(self) -> &'static str {
        match self {
            Category::Baka => "baka",
            Category::Cry => "cry",
            Category::Cuddle => "cuddle",
            Category::Dance => "dance",
            Category::Feed => "feed",
            Category::Hug => "hug",
            Category::Kiss => "kiss",
            Category::Laugh => "laugh",
            Category::Nekos => "nekos",
            Category::Pat => "pat",
            Category::Poke => "poke",
            Category::Slap => "slap",
            Category::Smile => "smile",
            Category::Smug => "smug",
            Category::Tickle => "tickle",
            Category::Wave => "wave",

            Category::Bite => "bite",
            Category::Blush => "blush",
            Category::Bored => "bored",
            Category::Facepalm => "facepalm",
            Category::Happy => "happy",
            Category::Highfive => "highfive",
            Category::Pout => "pout",
            Category::Shrug => "shrug",
            Category::Sleep => "sleep",
            Category::Stare => "stare",
            Category::Think => "think",
            Category::ThumbsUp => "thumbsup",
            Category::Wink => "wink",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_url_path().fmt(f)
    }
}
