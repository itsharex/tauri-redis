use crate::connection::{CValue, Connectable, Connection, Manager};
use crate::err::{self, CusError};
use crate::pubsub::{PubsubItem, PubsubManager};
use crate::response::EventResp;
use crate::sqlite::Connection as ConnectionModel;
use crate::utils;
use chrono::format;
use futures::future::OkInto;
use futures::stream::StreamExt;
use futures::FutureExt;
use rand::Error;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, AsyncConnectionConfig, Commands, Connection as RedisConnection, FromRedisValue, PushInfo, RedisError};
use serde::{Deserialize, Serialize};

use tauri::Emitter;
use tauri::State;

use tokio::sync::oneshot;

#[derive(Deserialize)]
struct SubscribeArgs {
    channels: Vec<String>,
}
#[derive(Serialize, Debug)]
struct Message {
    channel: String,
    payload: String,
}

pub async fn subscribe<'r>(
    window: tauri::Window,
    pubsub_manager: State<'r, PubsubManager>,
    payload: String,
    cid: u32,
) -> Result<String, CusError> {
    let args: SubscribeArgs = serde_json::from_str(&payload)?;

    let event_name = utils::random_str(32);
    let event_name_resp = event_name.clone();
    let model = ConnectionModel::first(cid)?;
    let (tx, rx) = oneshot::channel::<()>();
    // a channel to stop loop when frontend close the page
    let connection = Connection::new(model.get_params());
    pubsub_manager.add(
        event_name.clone(),
        PubsubItem::new(
            tx,
            event_name.clone(),
            connection.get_host(),
            "pubsub".to_string(),
            connection.get_proxy(),
        ),
    );
    tokio::spawn(async move {
        let conn_result = connection.get_sync_one().await;
        match conn_result {
            Err(e) => {
                println!("{}", e)
            }
            Ok(mut conn) => {
                let mut subpub = conn.as_pubsub();
                for x in args.channels {
                    let result = subpub.subscribe(&x);
                    match result {
                        Err(e) => {
                            println!("{}", e)
                        }
                        Ok(_) => {
                            println!("ok")
                        }
                    }
                }
                let event_str = event_name.as_str();
                tokio::select! {
                    _ = async {
                            loop {
                                let msg = subpub.get_message();
                                if let Ok(msg_payload) = msg {
                                    let payload = msg_payload.get_payload();
                                    if let Ok(result) = payload {
                                        println!("收到消息: {}", result);
                                        let r: EventResp<Message> = EventResp::new(
                                            Message {
                                                channel: msg_payload.get_channel_name().to_string(),
                                                payload: result,
                                            },
                                            String::from(event_str),
                                        );
                                        let _ = window.emit(event_str, serde_json::to_string(&r).unwrap());
                                    }
                                }
                            }
                    } => {},
                    _ = rx => {
                        println!("terminating accept loop");
                    }
                }
            }
        }
        drop(connection);
    });
    Ok(event_name_resp)
}

#[derive(Deserialize)]
struct PublishArgs {
    db: u8,
    channel: String,
    value: String,
}

pub async fn publish<'r>(
    payload: String,
    cid: u32,
    manager: tauri::State<'r, Manager>,
) -> Result<i64, CusError> {
    let args: PublishArgs = serde_json::from_str(&payload)?;
    manager
        .execute(
            cid,
            redis::cmd("publish").arg(args.channel).arg(args.value),
            Some(args.db),
        )
        .await
}

pub async fn monitor<'r>(
    window: tauri::Window,
    pubsub_manager: State<'r, PubsubManager>,
    cid: u32,
) -> Result<String, CusError> {
    let model = ConnectionModel::first(cid)?;
    let mut connection = Connection::new(model.get_params());
    let mut conn: RedisConnection = connection.get_sync_one().await?;

    let event_name = utils::random_str(32);
    let event_name_resp = event_name.clone();

    // // a channel to stop loop when frontend close the page
    let (tx, rx) = oneshot::channel::<()>();
    pubsub_manager.add(
        event_name_resp.clone(),
        PubsubItem::new(
            tx,
            event_name.clone(),
            connection.get_host(),
            "monitor".to_string(),
            connection.get_proxy(),
        ),
    );
    tokio::spawn(async move {
        let event_str = event_name.as_str();
        let v: Result<CValue, RedisError> = redis::cmd("CLIENT")
            .arg("SETNAME")
            .arg("monitor")
            .query(&mut conn);
        let result = conn.send_packed_command(b"monitor");
        println!("{:?}", result);
        match result {
            Ok(()) => {
                tokio::select! {
                    _ = async {
                        loop {
                            let r = conn.recv_response();
                            match r {
                                Ok(rr) => {
                                    println!("{:?}", rr)
                                },
                                Err(e) => {
                                    println!("{:?}", e)

                                }
                            }
                        }
                    } => {
                    }
                    _ = rx => {
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    });
    Ok(event_name_resp)
}

#[derive(Deserialize)]
struct CancelArgs {
    name: String,
}
pub async fn cancel<'r>(
    payload: String,
    pubsub_manager: State<'r, PubsubManager>,
) -> Result<String, CusError> {
    let args: CancelArgs = serde_json::from_str(&payload)?;
    pubsub_manager.close(&args.name);
    Ok(String::from("OK"))
}
