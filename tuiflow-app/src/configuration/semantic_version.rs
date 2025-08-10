use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct SemanticVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}.{}.{}", self.major, self.minor, self.patch).as_str())
    }
}

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "SemanticVersion",
            &["major", "minor", "patch"],
            SemanticVersionVisitor,
        )
    }
}

struct SemanticVersionVisitor;

impl<'de> serde::de::Visitor<'de> for SemanticVersionVisitor {
    type Value = SemanticVersion;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a semantic version identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let parts: Vec<&str> = v.split('.').collect();
        let major = parts
            .get(0)
            .ok_or(E::custom("Missing major version"))?
            .parse::<u32>()
            .map_err(|e| E::custom(format!("Invalid major version: {}", e)))?;

        let minor = parts
            .get(1)
            .ok_or(E::custom("Missing minor version"))?
            .parse::<u32>()
            .map_err(|e| E::custom(format!("Invalid minor version: {}", e)))?;
        let patch = parts
            .get(2)
            .ok_or(E::custom("Missing patch version"))?
            .parse::<u32>()
            .map_err(|e| E::custom(format!("Invalid patch version: {}", e)))?;
        Ok(SemanticVersion {
            major,
            minor,
            patch,
        })
    }
}
