use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PortsStmt(pub Vec<PortItem>);

impl<'de> serde::Deserialize<'de> for PortsStmt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        Ok(Self(Vec::deserialize(deserializer)?))
    }
}

/// The network protocol for a port binding.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum PortProtocol {
    #[default]
    Tcp,
    Udp,
}

impl PortProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

/// A single port mapping in short form, matching Docker Compose short syntax.
///
/// Supported formats:
/// - `HOST_PORT:CONTAINER_PORT`
/// - `HOST_IP:HOST_PORT:CONTAINER_PORT`
/// - Any of the above with `/tcp` or `/udp` suffix
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortItem {
    pub host_ip: Option<String>,
    pub host_port: String,
    pub container_port: String,
    pub protocol: PortProtocol,
}

#[derive(Debug, thiserror::Error)]
#[error(
    "port item must be in the format '[HOST_IP:][HOST_PORT:]CONTAINER_PORT[/PROTOCOL]', provided: {0}"
)]
pub struct PortItemParsingError(String);

impl FromStr for PortItem {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split off optional /protocol suffix
        let (port_spec, protocol) = match s.rsplit_once('/') {
            Some((spec, "tcp")) => (spec, PortProtocol::Tcp),
            Some((spec, "udp")) => (spec, PortProtocol::Udp),
            Some(_) => color_eyre::eyre::bail!(PortItemParsingError(s.to_string())),
            None => (s, PortProtocol::Tcp),
        };

        // Split into at most 3 parts on ':'.
        // Examples:
        //   "8080:80"             → ["8080", "80"]
        //   "127.0.0.1:8001:8001" → ["127.0.0.1", "8001", "8001"]
        let parts: Vec<&str> = port_spec.splitn(3, ':').collect();
        match parts.as_slice() {
            [host_port, container_port] if !host_port.is_empty() && !container_port.is_empty() => {
                Ok(Self {
                    host_ip: None,
                    host_port: host_port.to_string(),
                    container_port: container_port.to_string(),
                    protocol,
                })
            },
            [host_ip, host_port, container_port]
                if !host_ip.is_empty() && !host_port.is_empty() && !container_port.is_empty() =>
            {
                Ok(Self {
                    host_ip: Some(host_ip.to_string()),
                    host_port: host_port.to_string(),
                    container_port: container_port.to_string(),
                    protocol,
                })
            },
            _ => color_eyre::eyre::bail!(PortItemParsingError(s.to_string())),
        }
    }
}

impl<'de> serde::Deserialize<'de> for PortItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        // Accept both quoted strings ("8080:80") and bare integers (3000) from YAML.
        struct PortItemVisitor;

        impl serde::de::Visitor<'_> for PortItemVisitor {
            type Value = PortItem;

            fn expecting(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                write!(
                    f,
                    "a port mapping string like '8080:80' or a bare port number like 3000"
                )
            }

            fn visit_str<E: serde::de::Error>(
                self,
                v: &str,
            ) -> Result<PortItem, E> {
                v.parse().map_err(serde::de::Error::custom)
            }

            fn visit_u64<E: serde::de::Error>(
                self,
                v: u64,
            ) -> Result<PortItem, E> {
                v.to_string().parse().map_err(serde::de::Error::custom)
            }

            fn visit_i64<E: serde::de::Error>(
                self,
                v: i64,
            ) -> Result<PortItem, E> {
                v.to_string().parse().map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(PortItemVisitor)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    // Success cases
    #[test_case("8080:80" => PortItem { host_ip: None, host_port: "8080".to_string(), container_port: "80".to_string(), protocol: PortProtocol::Tcp } ; "host and container port")]
    #[test_case("127.0.0.1:8001:8001" => PortItem { host_ip: Some("127.0.0.1".to_string()), host_port: "8001".to_string(), container_port: "8001".to_string(), protocol: PortProtocol::Tcp } ; "ip host and container port")]
    #[test_case("6060:6060/udp" => PortItem { host_ip: None, host_port: "6060".to_string(), container_port: "6060".to_string(), protocol: PortProtocol::Udp } ; "udp protocol")]
    fn test_port_item_parsing_success(input: &str) -> PortItem {
        PortItem::from_str(input).expect("Should parse successfully")
    }

    // Failure cases
    #[test_case("" ; "empty string")]
    #[test_case(":80" ; "empty host port with no ip")]
    #[test_case("8080:80/ftp" ; "unsupported protocol")]
    fn test_port_item_parsing_failure(input: &str) {
        let result = PortItem::from_str(input);
        assert!(
            result.is_err(),
            "Input '{input}' should have failed parsing"
        );
    }
}
