use std::str::FromStr;

const DOCKER_IMAGE_TAG_DELIMETER: char = ':';

#[derive(Debug, PartialEq, Eq)]
pub struct DockerImageDef {
    pub image: String,
    pub tag: Option<String>,
}

impl FromStr for DockerImageDef {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.split_once(DOCKER_IMAGE_TAG_DELIMETER) {
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
