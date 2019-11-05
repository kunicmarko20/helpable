#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "_class")]
pub enum Action {
    #[serde(rename = "hudson.plugins.git.util.BuildData")]
    BuildData {
        last_build_revision: LastBuildRevision
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct LastBuildRevision {
    pub sha1: String,
}
