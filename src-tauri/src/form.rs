use ::redis as rds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionForm {
    pub id: Option<u32>,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub username: Option<String>,
    pub is_cluster: bool,
    pub readonly: bool,
    pub ssh_host: Option<String>,
    pub ssh_port: Option<u16>,
    pub ssh_password: Option<String>,
    pub ssh_username: Option<String>,
    pub ssh_private_key: Option<String>,
    pub ssh_timeout: Option<u32>,
    pub ssh_passphrase: Option<String>,
}

impl rds::IntoConnectionInfo for ConnectionForm {
    fn into_connection_info(self) -> rds::RedisResult<::redis::ConnectionInfo> {
        Ok(::redis::ConnectionInfo {
            addr: rds::ConnectionAddr::Tcp(self.host.clone(), self.port),
            redis: rds::RedisConnectionInfo {
                db: 0,
                username: None,
                password: self.password,
            },
        })
    }
}
