use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn get_example(
    get_data: HashMap<String, String>,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    println!("GET请求示例……");

    // let get_string = format!("GET:{:#?}", get_date);
    println!("GET:{:#?}", get_data);

    let html = "GET请求示例".to_string();
    Ok(warp::reply::html(html))
}

// 数值类型要对应
pub async fn get_example_number(
    get_data: HashMap<String, i64>,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    println!("GET请求示例number……");

    // let get_string = format!("GET:{:#?}", get_date);
    println!("GET:{:#?}", get_data);

    let html = "GET请求示例".to_string();
    Ok(warp::reply::html(html))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyObject {
    pub key1: String,
    pub key2: u32,
}

//get参数必需有，可多不可少，但参数类型必需对
// get /example2?key1=value&key2=42
pub async fn example(get_data: MyObject) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    println!("自定义结构体的GET:{:#?}", get_data);
    let html = "自定义结构体的GET请求示例".to_string();
    Ok(warp::reply::html(html))
}

//get参数可选 ?key1=value&key2=42
pub async fn my_options(
    get: Option<MyObject>,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    println!("可选GET参数自定义结构体的GET:{:#?}", get);
    let html = "可选GET参数GET请求示例".to_string();
    Ok(warp::reply::html(html))
}
