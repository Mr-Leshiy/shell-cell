use std::{fmt::Display, str::FromStr};

const IMAGE_TAG_DELIMETER: char = ':';

#[derive(Debug, thiserror::Error)]
#[error("Image must be in the format '<image_name>[:<tag>]', provided: {0}")]
pub struct ImageDefParsingError(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ImageDef {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        color_eyre::eyre::ensure!(!str.is_empty(), ImageDefParsingError(str.to_string()));
        match str.split_once(IMAGE_TAG_DELIMETER) {
            Some((prefix, suffix)) => {
                color_eyre::eyre::ensure!(
                    !prefix.is_empty(),
                    ImageDefParsingError(str.to_string())
                );
                color_eyre::eyre::ensure!(
                    !suffix.is_empty(),
                    ImageDefParsingError(str.to_string())
                );
                color_eyre::eyre::ensure!(
                    !suffix.contains(IMAGE_TAG_DELIMETER),
                    ImageDefParsingError(str.to_string())
                );
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

impl<'de> serde::Deserialize<'de> for ImageDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        str.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    // Success cases
    #[test_case("image" => ImageDef {
        image: "image".to_string(),
        tag: None
    } ; "only image name")]
    #[test_case("image:tag" => ImageDef {
        image: "image".to_string(),
        tag: Some("tag".to_string())
    } ; "image name with tag")]
    #[test_case("ubuntu:20.04" => ImageDef {
        image: "ubuntu".to_string(),
        tag: Some("20.04".to_string())
    } ; "image with version tag")]
    #[test_case("my-image:latest" => ImageDef {
        image: "my-image".to_string(),
        tag: Some("latest".to_string())
    } ; "image with hyphen and latest tag")]
    #[test_case("registry.io/org/image:v1.2.3" => ImageDef {
        image: "registry.io/org/image".to_string(),
        tag: Some("v1.2.3".to_string())
    } ; "full registry path with semver tag")]
    #[test_case("my_image" => ImageDef {
        image: "my_image".to_string(),
        tag: None
    } ; "image with underscore no tag")]
    fn test_image_def_parsing_success(input: &str) -> ImageDef {
        ImageDef::from_str(input).expect("Should parse successfully")
    }

    // Failure cases
    #[test_case("image:" ; "image with empty tag")]
    #[test_case(":tag" ; "empty image with tag")]
    #[test_case("" ; "empty string")]
    #[test_case("a:b:c" ; "multiple colons")]
    fn test_image_def_parsing_failure(input: &str) {
        let result = ImageDef::from_str(input);
        assert!(
            result.is_err(),
            "Input '{input}' should have failed parsing"
        );
    }

    // Display roundtrip
    #[test_case("image" ; "image without tag roundtrips")]
    #[test_case("image:tag" ; "image with tag roundtrips")]
    #[test_case("registry.io/org/image:v1.2.3" ; "full registry path roundtrips")]
    fn test_image_def_display_roundtrip(input: &str) {
        let parsed = ImageDef::from_str(input).expect("Should parse successfully");
        assert_eq!(parsed.to_string(), input);
    }
}
