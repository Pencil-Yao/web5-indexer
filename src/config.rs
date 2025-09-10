use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub data_base_url: String,
    pub ckb_node: String,
    pub ckb_network: String,
    pub listen_port: u64,
    pub log_level: String,
    pub worker_num: u64,
    pub start_height: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let log_level = env::var("LOG_LEVEL").unwrap_or("info".to_string());
        Self {
            data_base_url: env::var("DATABASE_URL")
                .unwrap_or("postgres://pg:password@127.0.0.1:5433/postgres".into()),
            ckb_node: env::var("CKB_NODE").unwrap_or("https://testnet.ckb.dev".into()),
            ckb_network: env::var("CKB_NETWORK").unwrap_or("ckb_testnet".into()),
            listen_port: env_int("LISTEN_PORT").unwrap_or(9533),
            log_level,
            worker_num: env_int("WORKER_NUM").unwrap_or(2),
            start_height: env_int("START_HEIGHT").unwrap_or(17_993_051),
        }
    }
}

pub fn env_int(name: &str) -> Option<u64> {
    match env::var(name) {
        Ok(str) => match str.parse::<u64>() {
            Ok(int) => Some(int),
            _ => None,
        },
        _ => None,
    }
}
