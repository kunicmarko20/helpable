use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct BuildData {
    pub sha1: String,
}

pub(super) fn deserialize_build_data<'de, D>(deserializer: D) -> Result<Option<BuildData>, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialized = Value::deserialize(deserializer);

    if deserialized.is_err() {
        return Ok(None);
    }

    let values = deserialized.unwrap();
    let values = values.as_array();

    if values.is_none() {
        return Ok(None);
    }

    let values = values.unwrap();

    for value in values {
        if value["_class"] != "hudson.plugins.git.util.BuildData" {
            continue;
        }

        let sha1 = &value["lastBuiltRevision"]["SHA1"];

        if sha1.is_null() || !sha1.is_string() {
            return Ok(None);
        }

        return Ok(Some(BuildData {
            sha1: sha1.as_str().unwrap().to_owned(),
        }));
    }

    Ok(None)
}
