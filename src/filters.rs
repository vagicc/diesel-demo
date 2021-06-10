use crate::handlers;
use crate::models::user_model;
use warp::Filter;

pub fn all_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let home = warp::get().and(warp::path::end()).map(|| format!("首页"));
    let users = warp::get()
        .and(warp::path!("users" / "distinct"))
        .and(warp::path::end())
        .map(|| {
            println!("`DISTINCT`关键字查询");
            use crate::models::user_model;
            user_model::distinct_test();
            format!("`DISTINCT`关键字查询")
        });
    let user_select = warp::get()
        .and(warp::path!("users" / "select"))
        .and(warp::path::end())
        .map(|| {
            println!("测试select");
            use crate::models::user_model;
            user_model::demo_select();
            format!("测试select")
        });
    let user_join = warp::get()
        .and(warp::path!("users" / "join"))
        .and(warp::path::end())
        .map(|| {
            println!("测试join联表查询");
            use crate::models::user_model;
            user_model::demo_join();
            format!("测试join联表查询")
        });
    let user_count = warp::get()
        .and(warp::path!("users" / "count"))
        .and(warp::path::end())
        .map(|| {
            println!("统计users表数据");
            crate::models::user_model::demo_count();
            format!("统计users表数据")
        });
    let user_inner_join = warp::get()
        .and(warp::path!("users" / "inner_join"))
        .and(warp::path::end())
        .map(|| {
            user_model::inner_join_example();
            format!("测试inner_join联表查询")
        });

    let user_filter = warp::get()
        .and(warp::path!("users" / "filter"))
        .and(warp::path::end())
        .map(|| {
            println!("filter_example");
            user_model::filter_example();
            format!("filter_example")
        });

    let animal = warp::get()
        .and(warp::path!("animal" / "distinct-on"))
        .and(warp::path::end())
        .and_then(handlers::animal_handler::distinct_demo);

    let routes = home
        .or(users)
        .or(user_select)
        .or(user_join)
        .or(user_count)
        .or(user_inner_join)
        .or(user_filter)
        .or(animal);
    routes
}
