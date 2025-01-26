use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `HDFSArtifact` is the location of an HDFS artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HDFSArtifact {
    /// `Addresses` is accessible addresses of HDFS name nodes
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,

    /// `Force` copies a file forcibly even if it exists
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// `HDFSUser` is the user to access HDFS file system.
    /// It is ignored if either ccache or keytab is used.
    #[serde(rename = "hdfsUser", skip_serializing_if = "Option::is_none")]
    pub hdfs_user: Option<String>,

    #[serde(rename = "krbCCacheSecret", skip_serializing_if = "Option::is_none")]
    pub krb_c_cache_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "krbConfigConfigMap", skip_serializing_if = "Option::is_none")]
    pub krb_config_config_map: Option<Box<core::v1::ConfigMapKeySelector>>,

    #[serde(rename = "krbKeytabSecret", skip_serializing_if = "Option::is_none")]
    pub krb_keytab_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `KRBRealm` is the Kerberos realm used with Kerberos keytab.
    /// It must be set if keytab is used.
    #[serde(rename = "krbRealm", skip_serializing_if = "Option::is_none")]
    pub krb_realm: Option<String>,

    /// `KRBServicePrincipalName` is the principal name of Kerberos service.
    /// It must be set if either ccache or keytab is used.
    #[serde(
        rename = "krbServicePrincipalName",
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_service_principal_name: Option<String>,

    /// `KRBUsername` is the Kerberos username used with Kerberos keytab.
    /// It must be set if keytab is used.
    #[serde(rename = "krbUsername", skip_serializing_if = "Option::is_none")]
    pub krb_username: Option<String>,

    /// `Path` is a file path in HDFS
    #[serde(rename = "path")]
    pub path: String,
}

impl HDFSArtifact {
    pub fn new(path: &str) -> Self {
        HDFSArtifact {
            path,
            ..Default::default()
        }
    }
}
