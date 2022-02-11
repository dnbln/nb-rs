type E = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), E> {
    #[allow(unused_variables)]
    let client = reqwest::Client::new();
    #[cfg(feature = "local")]
    {
        use std::{collections::HashMap, convert::TryFrom, path::PathBuf, str::FromStr};

        use proc_macro2::TokenStream;
        use quote::{format_ident, quote, ToTokens, TokenStreamExt};

        #[derive(serde::Deserialize)]
        struct EndpointDescInternal {
            min: String,
            max: String,
            format: String,
        }

        #[derive(serde::Deserialize)]
        #[serde(try_from = "EndpointDescInternal")]
        struct EndpointDesc {
            min: usize,
            max: usize,
            format: String,
            with_padding: usize,
        }

        impl TryFrom<EndpointDescInternal> for EndpointDesc {
            type Error = <usize as FromStr>::Err;

            fn try_from(d: EndpointDescInternal) -> Result<Self, Self::Error> {
                Ok(Self {
                    min: d.min.parse()?,
                    max: d.max.parse()?,
                    format: d.format,
                    with_padding: d.max.len(),
                })
            }
        }

        const BASE_URL: &str = "https://nekos.best/api/v2";

        async fn get_endpoints_data(
            client: &reqwest::Client,
        ) -> Result<HashMap<String, EndpointDesc>, E> {
            if let Ok(_) = std::env::var("DOCS_RS") {
                // we are on docs.rs

                let endpoints_json = r#"{"baka":{"min":"001","max":"014","format":"gif"},"bite":{"min":"001","max":"013","format":"gif"},"blush":{"min":"001","max":"013","format":"gif"},"bored":{"min":"001","max":"015","format":"gif"},"cry":{"min":"001","max":"040","format":"gif"},"cuddle":{"min":"001","max":"028","format":"gif"},"dance":{"min":"001","max":"021","format":"gif"},"facepalm":{"min":"001","max":"011","format":"gif"},"feed":{"min":"001","max":"023","format":"gif"},"happy":{"min":"001","max":"012","format":"gif"},"highfive":{"min":"001","max":"013","format":"gif"},"hug":{"min":"001","max":"024","format":"gif"},"kiss":{"min":"001","max":"028","format":"gif"},"laugh":{"min":"001","max":"019","format":"gif"},"nekos":{"min":"0001","max":"0476","format":"jpg"},"pat":{"min":"001","max":"038","format":"gif"},"poke":{"min":"001","max":"021","format":"gif"},"pout":{"min":"001","max":"008","format":"gif"},"shrug":{"min":"001","max":"008","format":"gif"},"slap":{"min":"001","max":"031","format":"gif"},"sleep":{"min":"001","max":"012","format":"gif"},"smile":{"min":"001","max":"023","format":"gif"},"smug":{"min":"001","max":"015","format":"gif"},"stare":{"min":"001","max":"014","format":"gif"},"think":{"min":"001","max":"012","format":"gif"},"thumbsup":{"min":"001","max":"016","format":"gif"},"tickle":{"min":"001","max":"021","format":"gif"},"wave":{"min":"001","max":"028","format":"gif"},"wink":{"min":"001","max":"015","format":"gif"}}"#;

                return Ok(serde_json::from_str(endpoints_json)?);
            }

            Ok(client
                .get(format!("{}/endpoints", BASE_URL))
                .send()
                .await?
                .json()
                .await?)
        }

        let endpoints = get_endpoints_data(&client).await?;

        struct CategoryData<'a>(&'a str, &'a EndpointDesc);

        impl<'a> ToTokens for CategoryData<'a> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let name = match self.0 {
                    "thumbsup" => format_ident!("ThumbsUp"),
                    _ => format_ident!("{}{}", self.0[..1].to_uppercase(), &self.0[1..]),
                };
                let min = self.1.min;
                let max = self.1.max;
                let format = &self.1.format;
                let with_padding = self.1.with_padding;

                tokens.append_all(quote! {
                    pub struct #name;

                    impl super::LocalNekosBestCategory for #name {
                        const CATEGORY: crate::Category = crate::Category::#name;
                        const MIN: usize = #min;
                        const MAX: usize = #max;
                        const WITH_PADDING: usize = #with_padding;
                        const FORMAT: &'static str = #format;
                    }

                    impl #name {
                        pub fn get_random(&self, random: usize) -> String {
                            <Self as super::LocalNekosBestCategory>::get_random(self, random)
                        }

                        pub fn get(&self) -> String {
                            <Self as super::LocalNekosBestCategory>::get(self)
                        }
                    }
                })
            }
        }

        let implementations = endpoints
            .into_iter()
            .map(|(name, endpoint)| {
                let category_data = CategoryData(&name, &endpoint);

                quote! {#category_data}
            })
            .collect::<TokenStream>();

        let implementations = implementations.to_string();

        std::fs::write(
            PathBuf::from(std::env::var("OUT_DIR")?).join("local_implementation.rs"),
            implementations,
        )?;
    }

    Ok(())
}
