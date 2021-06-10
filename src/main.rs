use diesel::QueryDsl;
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use warp::Filter;

mod db;
mod handlers;
mod models;
mod schema;
mod filters;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    let routes = filters::all_routes();

    let cert_path = get_env("cert_path");
    let key_path = get_env("key_path");
    let ip_addr = get_env("ip_address");
    let socket_addr: SocketAddr = ip_addr.as_str().parse().unwrap();

    println!("warp https 监听： {:?}", ip_addr);

    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(socket_addr)
        .await;
}


pub fn get_env(key: &str) -> String {
    dotenv().ok();
    let msg = ".env文件必须配置的环境变量： ".to_string() + key;
    let value = env::var(key).expect(&msg);
    value
}
