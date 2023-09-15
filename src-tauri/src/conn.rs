use crate::{
    err::CusError,
    model::{Command, Node},
    response,
    ssh::{self, SshTunnel},
    utils,
};
use chrono::prelude::*;
use redis::aio::{Connection, ConnectionLike};
use redis::cluster::ClusterClient;
use redis::cluster_async::ClusterConnection;
use redis::Client;
use redis::Cmd;
use redis::{Arg, FromRedisValue, Value};
use ssh_jumper::model::SshForwarderEnd;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::oneshot::Receiver;
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::time::timeout;

#[derive(Clone, Debug)]
pub struct RedisParam {
    pub tcp_host: String,
    pub tcp_port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_cluster: bool,
}

impl redis::IntoConnectionInfo for RedisParam {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        Ok(redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(self.tcp_host.clone(), self.tcp_port),
            redis: redis::RedisConnectionInfo {
                db: 0,
                username: self.username,
                password: self.password,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct RedisConnectionParams {
    pub redis_params: RedisParam,
    pub ssh_params: Option<ssh::SshParams>,
    pub model_name: String,
    pub is_cluster: bool,
}

pub trait Connectable {
    fn get_params(&self) -> RedisConnectionParams;
}

pub struct RedisConnection {
    pub params: RedisConnectionParams,
    pub cancel_tunnel_rx: Option<Receiver<SshForwarderEnd>>,
    pub tunnel_addr: Option<SocketAddr>,
}

impl ssh::SshTunnel for RedisConnection {
    fn get_ssh_config(&self) -> Option<ssh::SshParams> {
        self.params.ssh_params.clone()
    }
    fn store_addr(&mut self, addr: SocketAddr, rx: Receiver<SshForwarderEnd>) {
        self.cancel_tunnel_rx = Some(rx);
        self.tunnel_addr = Some(addr);
    }
    fn close_tunnel(&mut self) {
        if let Some(mut rx) = self.cancel_tunnel_rx.take() {
            rx.close();
        }
    }
}
impl Connectable for RedisConnection {
    fn get_params(&self) -> RedisConnectionParams {
        self.params.clone()
    }
}
impl Drop for RedisConnection {
    fn drop(&mut self) {
        self.close_tunnel();
    }
}
impl RedisConnection {
    pub fn build(params: RedisConnectionParams) -> Self {
        Self {
            params: params,
            cancel_tunnel_rx: None,
            tunnel_addr: None,
        }
    }
    pub fn get_proxy(&self) -> Option<String> {
        if let Some(addr) = self.tunnel_addr {
            return Some(format!("{}:{}", addr.ip().to_string(), addr.port()));
        }
        return None;
    }
    pub fn get_is_cluster(&self) -> bool {
        self.params.is_cluster
    }
    pub fn get_host(&self) -> String {
        format!(
            "redis://{}:{}",
            self.params.redis_params.tcp_host.clone(),
            self.params.redis_params.tcp_port
        )
    }
    pub fn get_redis_params(&self) -> RedisParam {
        let mut params = self.params.redis_params.clone();
        if let Some(addr) = self.tunnel_addr {
            params.tcp_host = addr.ip().to_string();
            params.tcp_port = addr.port();
        }
        params
    }

    pub async fn get_normal(&mut self) -> Result<Connection, CusError> {
        ssh::create_tunnel(self).await?;
        let params = self.get_redis_params();
        let client = Client::open(params)?;
        let rx = timeout(Duration::from_secs(2), client.get_async_connection()).await;
        match rx {
            Ok(conn_result) => match conn_result {
                Ok(connection) => {
                    return Ok(connection);
                }
                Err(e) => {
                    return Err(CusError::App(e.to_string()));
                }
            },
            Err(_) => {
                return Err(CusError::App(String::from("Connection Timeout")));
            }
        }
    }
    pub async fn get_cluster(&mut self) -> Result<ClusterConnection, CusError> {
        ssh::create_tunnel(self).await?;
        let params = self.get_redis_params();
        let client = ClusterClient::new(vec![params]).unwrap();
        let rx = timeout(Duration::from_secs(2), client.get_async_connection()).await;
        match rx {
            Ok(conn_result) => match conn_result {
                Ok(connection) => {
                    return Ok(connection);
                }
                Err(e) => {
                    return Err(CusError::App(e.to_string()));
                }
            },
            Err(_) => {
                return Err(CusError::App(String::from("Connection Timeout")));
            }
        }
    }
}

pub struct ConnectionWrapper {
    pub conn: Box<dyn ConnectionLike + Send>,
    pub id: String,
    pub nodes: Vec<Node>,
    pub db: u8,
    pub created_at: chrono::DateTime<Local>,
    pub model: RedisConnection,
}

impl ConnectionWrapper {
    pub async fn build<T: Connectable>(model: T) -> Result<Self, CusError> {
        let b: Box<dyn ConnectionLike + Send>;
        let params: RedisConnectionParams = model.get_params();
        let mut connection = RedisConnection::build(params);
        let cluster = connection.params.is_cluster;
        if cluster {
            b = Box::new(connection.get_cluster().await?)
        } else {
            b = Box::new(connection.get_normal().await?);
        }
        let r = Self {
            id: utils::random_str(32),
            nodes: vec![],
            db: 0,
            created_at: Local::now(),
            model: connection,
            conn: b,
        };
        Ok(r)
    }

    // execute the redis command
    async fn execute(
        &mut self,
        cmd: &mut redis::Cmd,
    ) -> Result<(redis::Value, Command), (CusError, Command)> {
        let mut cmd_vec: Vec<String> = vec![];
        for arg in cmd.args_iter() {
            match arg {
                Arg::Simple(v) => match String::from_utf8(v.to_vec()) {
                    Ok(s) => {
                        cmd_vec.push(s);
                    }
                    Err(s) => {
                        cmd_vec.push(utils::binary_to_redis_str(&v.to_vec()));
                    }
                },
                Arg::Cursor => {}
            }
        }
        let start = Local::now();
        let value_r = cmd.query_async(self).await;
        let end = Local::now();
        let mut rep: Vec<String> = vec![];
        let mut cus_cmd = Command {
            id: utils::random_str(32),
            cmd: cmd_vec.join(" "),
            response: String::new(),
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            host: self.model.get_host(),
            duration: end.timestamp_micros() - start.timestamp_micros(),
        };
        match value_r {
            Ok(value) => {
                match &value {
                    Value::Bulk(v) => {
                        for vv in v {
                            match vv {
                                Value::Data(vvv) => {
                                    let s = String::from_utf8(vvv.to_vec()).unwrap();
                                    rep.push(s);
                                }
                                Value::Bulk(vvv) => {
                                    for vvvv in vvv {
                                        match vvvv {
                                            Value::Data(vvvvv) => {
                                                let s = String::from_utf8(vvvvv.to_vec()).unwrap();
                                                rep.push(s);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Value::Int(v) => rep.push(v.to_string()),
                    Value::Nil => rep.push(String::from("nil")),
                    Value::Data(v) => {
                        // maybe value is bitmap
                        let s = String::from_utf8(v.to_vec());
                        match s {
                            Ok(s) => rep.push(s),
                            Err(_) => {
                                let i: Vec<u8> = Vec::from_redis_value(&value).unwrap();
                                let binary = i
                                    .iter()
                                    .map(|u| format!("{:b}", u))
                                    .collect::<Vec<String>>()
                                    .join("");

                                rep.push(binary)
                            }
                        }
                    }
                    Value::Status(v) => rep.push(v.to_string()),
                    Value::Okay => {
                        rep.push(String::from("OK"));
                    }
                }
                cus_cmd.response = rep.join(" ");

                Ok((value, cus_cmd))
            }
            Err(err) => {
                rep.push(err.to_string());
                cus_cmd.response = rep.join(" ");
                Err((CusError::App(err.to_string()), cus_cmd))
            }
        }
    }
}

impl ConnectionLike for ConnectionWrapper {
    fn req_packed_command<'a>(
        &'a mut self,
        cmd: &'a redis::Cmd,
    ) -> redis::RedisFuture<'a, redis::Value> {
        self.conn.req_packed_command(cmd)
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a redis::Pipeline,
        offset: usize,
        count: usize,
    ) -> redis::RedisFuture<'a, Vec<redis::Value>> {
        self.conn.req_packed_commands(cmd, offset, count)
    }

    fn get_db(&self) -> i64 {
        self.conn.get_db()
    }
}

/**
 * connection manager state
 */
pub struct ConnectionManager {
    pub map: Mutex<HashMap<u32, ConnectionWrapper>>,
    debug_tx: Mutex<Vec<Sender<Command>>>,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            map: Mutex::new(HashMap::new()),
            debug_tx: Mutex::new(vec![]),
        }
    }
    pub async fn add(&self, id: u32, conn: ConnectionWrapper) {
        self.map.lock().await.insert(id, conn);
        if let Some(conn) = self.map.lock().await.get_mut(&id) {
            let _ = self.set_name(conn, "tauri-redis".to_string()).await;
        }
    }

    pub async fn set_name(
        &self,
        conn: &mut ConnectionWrapper,
        name: String,
    ) -> Result<(), CusError> {
        self.execute_with(redis::cmd("CLIENT").arg("SETNAME").arg(&name), conn)
            .await?;
        Ok(())
    }

    pub async fn get_config(
        &self,
        id: u32,
        pattern: &str,
    ) -> Result<HashMap<String, String>, CusError> {
        if let Some(conn) = self.map.lock().await.get_mut(&id) {
            return self.get_config_with(pattern, conn).await;
        }
        return Err(CusError::App(String::from("Connection Not Found")));
    }

    pub async fn get_config_with(
        &self,
        pattern: &str,
        conn: &mut ConnectionWrapper,
    ) -> Result<HashMap<String, String>, CusError> {
        let value: Value = self
            .execute_with(redis::cmd("config").arg("get").arg(pattern), conn)
            .await?;
        let vec: Vec<String> = Vec::from_redis_value(&value)?;
        let mut map = HashMap::new();

        let mut i: usize = 0;
        while i < vec.len() {
            if let Some(key) = vec.get(i) {
                if let Some(value) = vec.get(i + 1) {
                    map.insert(key.clone(), value.clone());
                }
            }
            i += 2;
        }
        Ok(map)
    }

    pub async fn get_version(&self, id: u32) -> Result<String, CusError> {
        if let Some(conn) = self.map.lock().await.get_mut(&id) {
            return self.get_version_with(conn).await;
        }
        return Err(CusError::App(String::from("Connection Not Found")));
    }

    // get redis server version
    pub async fn get_version_with(&self, conn: &mut ConnectionWrapper) -> Result<String, CusError> {
        let info = self.get_info_with(conn).await?;
        if let Some(fields) = info.get(0) {
            if let Some(version) = fields.get("redis_version") {
                return Ok(version.clone());
            }
        }
        Ok(String::from(""))
    }

    pub async fn get_info(&self, id: u32) -> Result<Vec<HashMap<String, String>>, CusError> {
        if let Some(conn) = self.map.lock().await.get_mut(&id) {
            return self.get_info_with(conn).await;
        }
        return Err(CusError::App(String::from("Connection Not Found")));
    }

    // get the server info
    // if the cluster server, response is vec
    // so for unify, normal server is change to vec, the value is set to vec
    pub async fn get_info_with(
        &self,
        conn: &mut ConnectionWrapper,
    ) -> Result<Vec<HashMap<String, String>>, CusError> {
        let v = self.execute_with(&mut redis::cmd("info"), conn).await?;
        let format_fn = |str_value: String| {
            let arr: Vec<&str> = str_value.split("\r\n").collect();
            let mut kv: HashMap<String, String> = HashMap::new();
            for v in arr {
                if v.contains(":") {
                    let key_value: Vec<&str> = v.split(":").collect();
                    if let Some(key) = key_value.get(0) {
                        if let Some(value) = key_value.get(1) {
                            kv.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
            return kv;
        };
        match v {
            Value::Data(cc) => {
                if let Ok(r) = String::from_utf8(cc) {
                    return Ok(vec![format_fn(r)]);
                }
            }
            Value::Bulk(vv) => {
                let mut r: Vec<HashMap<String, String>> = vec![];
                for vvv in vv {
                    r.push(format_fn(String::from_redis_value(&vvv)?));
                }
                return Ok(r);
            }
            _ => {}
        }
        return Err(CusError::App(String::from("Connected Timeout")));
    }

    // get cluster server nodes
    pub async fn get_nodes(&self, id: u32) -> Result<Vec<Node>, CusError> {
        if let Some(conn) = self.map.lock().await.get_mut(&id) {
            return Ok(self.get_nodes_with(conn).await?);
        }
        return Err(CusError::App(String::from("Connection Not Found")));
    }

    // get cluster server nodes
    pub async fn get_nodes_with(
        &self,
        wrapper: &mut ConnectionWrapper,
    ) -> Result<Vec<Node>, CusError> {
        if !wrapper.model.get_is_cluster() {
            return Err(CusError::App(String::from("Not a Cluster Server")));
        }
        if wrapper.nodes.len() == 0 {
            let params = wrapper.model.get_params();
            let values = self
                .execute_with(redis::cmd("CLUSTER").arg("nodes"), wrapper)
                .await?;
            let csv = String::from_redis_value(&values)?;
            let items: Vec<&str> = csv.split("\n").collect();
            let mut nodes: Vec<Node> = vec![];
            for ss in items {
                if ss.trim() != "" {
                    let node = Node::build(ss.to_string(), params.clone());
                    nodes.push(node)
                }
            }
            wrapper.nodes = nodes;
        }
        Ok(wrapper.nodes.to_vec())
    }

    pub async fn execute_with(
        &self,
        cmd: &mut Cmd,
        conn: &mut ConnectionWrapper,
    ) -> Result<redis::Value, CusError> {
        let result = conn.execute(cmd).await;
        match result {
            Ok((value, cmd)) => {
                if let Some(tx) = self.debug_tx.lock().await.get_mut(0) {
                    let _ = tx.send(cmd).await;
                }
                return Ok(value);
            }
            Err((err, cmd)) => {
                if let Some(tx) = self.debug_tx.lock().await.get_mut(0) {
                    let _ = tx.send(cmd).await;
                }
                return Err(err);
            }
        }
    }

    pub async fn execute(
        &self,
        id: u32,
        db: u8,
        cmd: &mut redis::Cmd,
    ) -> Result<redis::Value, CusError> {
        if let Some(conn) = self.map.lock().await.get_mut(&id) {
            if !conn.model.get_is_cluster() && conn.db != db {
                let _ = self
                    .execute_with(redis::cmd("select").arg(db), conn)
                    .await?;
                conn.db = db
            }
            let v = self.execute_with(cmd, conn).await?;
            return Ok(v);
        }
        return Err(CusError::App(String::from("Connection Not Found")));
    }

    pub async fn get_conns(&self) -> Vec<response::Conn> {
        let mut vec = vec![];
        for (_, v) in self.map.lock().await.iter() {
            vec.push(response::Conn {
                id: v.id.clone(),
                host: v.model.get_host(),
                created_at: v.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                types: "normal".to_string(),
                proxy: v.model.get_proxy(),
            })
        }
        vec
    }

    pub async fn remove(&self, id: u32) {
        self.map.lock().await.remove(&id);
    }

    pub async fn set_tx(&self, tx: Sender<Command>) {
        self.debug_tx.lock().await.insert(0, tx);
    }

    pub async fn remove_tx(&self) {
        self.debug_tx.lock().await.remove(0);
    }
}