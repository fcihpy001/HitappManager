mod config;

use crate::config::{load_config, update_config};
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::{Command};
use std::thread::spawn;
use axum::extract::Query;


//查看进程状态
async fn process_status(wallet: Query<Process>) -> impl IntoResponse {
    let config = load_config();
    println!("wallet:{}",config.clone().wallet);
    if wallet.wallet != config.wallet {
        return Json(Response {
            code: 400,
            msg: "钱包地址不一致，没有权限查看".to_string(),
            data: false,
        });
    }

    // 执行命令
    let process_name = "hitapp";
    let os = std::env::consts::OS;
    let mut arg = format!("ps -ef | grep {}", process_name);
    if os == "windows" {
        arg = format!("tasklist | findstr 'hitapp'");
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut status = output_str.contains("./hitapp");
    if os == "windows" {
        status = output_str.contains("hitapp.exe");
    }
    Json(Response {
        code: 0,
        msg: "success".to_string(),
        data: status,
    })
}

async fn process_start(Json(payload): Json<Process>) -> impl IntoResponse {

    let config = load_config();
    println!("config_info: {:?}", config.clone());
    if payload.wallet != config.wallet {
        return Json(Response {
            code: 400,
            msg: "fail".to_string(),
            data: "启动失败，钱包地址不一致",
        });
    }
    let process_name = "hitapp";
    if let Err(err) = env::set_current_dir(&config.dir) {
        eprintln!("Failed to change directory: {}", err);
        println!("目录为空：{}",err);
        return Json(Response {
            code: 400,
            msg: "请设置程序运行的目录".to_string(),
            data: "",
        });
    }

    let os = std::env::consts::OS;
    let mut  arg = format!("nohup ./{} &", process_name);
    if os == "windows" {
        arg = format!("start /B hitapp.exe");
    }
    spawn(move || {
      Command::new("sh")
            .arg("-c")
            .arg(arg)
            .output()
            .expect("failed to execute process")
    });

    Json(Response {
        code: 0,
        msg: "success".to_string(),
        data: "启动成功",
    })
}

async fn process_stop(Json(payload): Json<Process>) -> impl IntoResponse {
    let config = load_config();
    if payload.wallet != config.wallet {
        return Json(Response {
            code: 400,
            msg: "fail".to_string(),
            data: "启动失败，钱包地址不一致",
        });
    }
    let process_name = "hitapp";
    let os = std::env::consts::OS;
    let mut  arg = format!("pkill {}", process_name);
    if os == "windows" {
        arg = format!("taskkill /im hitapp.exe");
    }
    Command::new("sh")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed to execute process");
    Json(Response {
        code: 0,
        msg: "success".to_string(),
        data: "终止成功",
    })
}

async fn process_restart(Json(payload): Json<Process>) -> impl IntoResponse {
    let config = load_config();
    if payload.wallet != config.wallet {
        return Json(Response {
            code: 400,
            msg: "fail".to_string(),
            data: "启动失败，钱包地址不一致",
        });
    }
    let process_name = "hitapp";
    // 停止
    let os = std::env::consts::OS;
    let mut  arg = format!("pkill {}", process_name);
    if os == "windows" {
        arg = format!("taskkill /im hitapp.exe");
    }
    Command::new("sh")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed to execute process");

    if let Err(err) = env::set_current_dir(&config.dir) {
        eprintln!("Failed to change directory: {}", err);
    }
    // 启动
    let mut  start_arg = format!("nohup ./{} &", process_name);
    if os == "windows" {
        start_arg = format!("start /B hitapp.exe");
    }
    spawn(move || {
        Command::new("sh")
            .arg("-c")
            .arg(start_arg)
            .output()
            .expect("failed to execute process")
    });
    Json(Response {
        code: 0,
        msg: "success".to_string(),
        data: "终止成功",
    })
}

#[tokio::main()]
async fn main() {
    tracing_subscriber::fmt::init();
    // 启动webserver
    let app = Router::new()
        .route("/config/info", get(info))
        .route("/process/status", get(process_status))

        .route("/process/start", post(process_start))
        .route("/process/stop", post(process_stop))
        .route("/process/restart", post(process_restart))
        .route("/config/update", post(update));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn info() -> impl IntoResponse {
    let config = load_config();
    println!("config:{:?}", config.clone());
    let data = Response {
        code: 0,
        msg: "success".to_string(),
        data: config,
    };
    Json(data)
}

#[derive(Deserialize)]
struct Process {
    wallet: String,
}

async fn update(Json(payload): Json<UpdateConfig>) -> impl IntoResponse {
    let mut config = load_config();
    config.wallet = payload.wallet;

    update_config(config.clone());
    Json(Response {
        code: 0,
        msg: "success".to_string(),
        data: config,
    })
}

#[derive(Deserialize)]
struct UpdateConfig {
    wallet: String,
}

#[derive(Serialize, Deserialize)]
struct Response<T> {
    code: u64,
    msg: String,
    data: T,
}
