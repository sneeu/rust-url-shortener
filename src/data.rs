use chrono::prelude::*;
use redis::{self, from_redis_value};
use redis::{Commands, FromRedisValue};
use serde::{Deserialize, Serialize};
// use serde_json;

const REDIS_NAMESPACE: &str = "urls";
const REDIS_NAMESPACE_SEPARATOR: &str = ":";
const REDIS_ID_KEY: &str = "max_id";

#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    pub id: i64,
    pub url: String,
    created_at: DateTime<Utc>,
}

impl FromRedisValue for Url {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let s: String = from_redis_value(v)?;

        Ok(serde_json::from_str(s.as_str())?)
    }
}

fn namespace_key(key: String) -> String {
    format!("{}{}{}", REDIS_NAMESPACE, REDIS_NAMESPACE_SEPARATOR, key)
}

pub fn read_url(id: i64) -> redis::RedisResult<Url> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    con.get(namespace_key(id.to_string()))
}

pub fn create_url(url: String) -> redis::RedisResult<Url> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let next_id: i64 = con.incr(namespace_key(REDIS_ID_KEY.to_string()), 1)?;

    let the_url = Url {
        id: next_id,
        url,
        created_at: Utc::now(),
    };

    con.set(
        namespace_key(next_id.to_string()),
        &serde_json::to_string(&the_url)?,
    )?;

    read_url(next_id)
}
