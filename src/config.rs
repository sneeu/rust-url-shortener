use redis::Client;
use std::env;

const HOST_NAME_DEFAULT: &str = "127.0.0.1:8000";
const REDIS_URL_DEFAULT: &str = "127.0.0.1:6379";

pub struct Config {
    pub host_name: String,
    redis_url: String,
}

impl Config {
    pub fn read() -> Result<Config, ()> {
        let host_name_result = env::var("HOST_NAME");
        let redis_url_result = env::var("REDIS_URL");

        if let (Ok(host_name), Ok(redis_url)) = (host_name_result, redis_url_result) {
            Ok(Config {
                host_name,
                redis_url,
            })
        } else {
            Ok(Config {
                host_name: HOST_NAME_DEFAULT.to_string(),
                redis_url: REDIS_URL_DEFAULT.to_string(),
            })
        }
    }

    pub fn redis_connection(&self) -> redis::RedisResult<Client> {
        redis::Client::open(self.redis_url.clone())
    }
}
