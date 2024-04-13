use bson::doc;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use uuid::Uuid;

// 配置文件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    ///线程数
    pub threads: u32,
    // 链
    pub chain: String,
    /// 生成私钥方法
    pub method: String,
    /// 多长时间报告1次
    pub report: u32,
    pub wallet: String,
    pub uuid: String,
    pub cpus: u32,
    pub detail: bool,
    pub dir: String
}

pub fn update_config(config: Config) {
    let file_name = "config";

    // 创建 BSON 文档
    let config_doc = doc! {
        "threads": config.threads,
        "chain": config.chain,
        "method": config.method,
        "report": config.report,
        "cpus": config.cpus,
        "wallet": config.wallet,
        "uuid": config.uuid,
        "detail": config.detail,
        "dir": config.dir
    };
    // 将 BSON 文档转换为字节向量
    let bson_bytes = bson::to_vec(&config_doc).expect("Failed to serialize BSON");

    // 打开文件并将 BSON 数据写入
    let mut file = File::create(file_name).expect("Failed to create file");
    file.write_all(&bson_bytes)
        .expect("Failed to write data to file");
}
pub fn load_config() -> Config {
    let file_name = "config";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    // 尝试从文件中读取配置信息
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    return if buffer.is_empty() {
        // 加载一个默认配置
        let config = Config {
            threads: 1,
            chain: "eth".to_string(),
            method: "private".to_string(),
            report: 10,
            cpus: num_cpus::get() as u32,
            wallet: "uuu".to_string(),
            uuid: Uuid::new_v4().to_string(),
            detail: true,
            dir: std::env::current_dir().unwrap().to_string_lossy().to_string()
        };
        let config_clone = config.clone();

        // 创建 BSON 文档
        let config_doc = doc! {
            "threads": config_clone.threads,
            "chain": config_clone.chain,
            "method": config_clone.method,
            "report": config_clone.report,
            "cpus": config_clone.cpus,
            "wallet": config_clone.wallet,
            "uuid": config_clone.uuid,
            "detail": config_clone.detail,
            "dir": config_clone.dir
        };
        // 将 BSON 文档转换为字节向量
        let bson_bytes = bson::to_vec(&config_doc).expect("Failed to serialize BSON");

        // 打开文件并将 BSON 数据写入
        let mut file = File::create(file_name).expect("Failed to create file");
        file.write_all(&bson_bytes)
            .expect("Failed to write data to file");
        config
    } else {
        // 读取并解析
        let mut file = File::open(file_name).expect("Failed to open file");
        let mut bson_bytes = Vec::new();
        file.read_to_end(&mut bson_bytes)
            .expect("Failed to read data from file");

        // 解析 BSON 数据为结构体
        let config: Config =
            bson::from_slice(&bson_bytes).expect("Failed to parse BSON into struct");
        config
    };
}
