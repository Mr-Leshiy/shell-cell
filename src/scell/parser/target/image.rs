use std::{fmt::Display, str::FromStr};

const IMAGE_TAG_DELIMETER: char = ':';

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageDef {
    pub image: String,
    pub tag: Option<String>,
}

impl Display for ImageDef {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(tag) = &self.tag {
            write!(f, "{}:{}", self.image, tag)
        } else {
            write!(f, "{}", self.image)
        }
    }
}

impl FromStr for ImageDef {
    type Err = color_eyre::eyre::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.split_once(IMAGE_TAG_DELIMETER) {
            Some((prefix, suffix)) => {
                Ok(Self {
                    image: prefix.to_string(),
                    tag: Some(suffix.to_string()),
                })
            },
            None => {
                Ok(Self {
                    image: str.to_string(),
                    tag: None,
                })
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("image" => ImageDef { 
        image: "image".to_string(),
        tag: None
    } ; "only image name")]
    #[test_case("image:tag" => ImageDef { 
        image: "image".to_string(),
        tag: Some("tag".to_string()) 
    } ; "image name with tag")]
    fn parsing_test(input: &str) -> ImageDef {
        ImageDef::from_str(input).expect("Should be a valid input")
    }
}
