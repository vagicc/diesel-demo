use crate::handlers;
use crate::models::animal_model;
use crate::models::user_model;
use warp::Filter;

pub fn all_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let home = warp::get()
        .and(warp::path::end())
        .map(|| format!("首页-diesel-demo"));
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

    let user_find_example = warp::get()
        .and(warp::path!("users" / "find"))
        .and(warp::path::end())
        .map(|| {
            user_model::find_example();
            format!("find直接查询主键")
        });

    let user_order = warp::get()
        .and(warp::path!("users" / "order"))
        .and(warp::path::end())
        .map(|| {
            user_model::order_example();
            format!("order example")
        });
    let user_limit = warp::get()
        .and(warp::path!("users" / "limit"))
        .and(warp::path::end())
        .map(|| {
            user_model::limit_example();
            format!("limit example")
        });
    let user_offset = warp::get()
        .and(warp::path!("users" / "offset"))
        .and(warp::path::end())
        .map(|| {
            user_model::offset_example();
            format!("offset example")
        });
    let into_boxed_example = warp::get()
        .and(warp::path!("users" / "into_boxed"))
        .and(warp::path::end())
        .map(|| {
            user_model::into_boxed_example();
            format!("into_boxed_example")
        });

    let for_update_example = warp::get()
        .and(warp::path!("users" / "for_update"))
        .and(warp::path::end())
        .map(|| {
            user_model::for_update_example();
            format!("悲观锁（for updata）")
        });
    let skip_locked = warp::get()
        .and(warp::path!("usres" / "skip_locked"))
        .and(warp::path::end())
        .map(|| {
            //测试失败
            println!("skip_locked START");
            user_model::skip_locked_example();
            format!("skip_locked_example")
        });

    let no_wait_example = warp::get()
        .and(warp::path!("users" / "no_wait"))
        .and(warp::path::end())
        .map(|| {
            user_model::no_wait_example();
            format!("no_wait_example")
        });

    let for_no_key_update = warp::get()
        .and(warp::path!("users" / "for_no_key_update"))
        .and(warp::path::end())
        .map(|| {
            user_model::for_no_key_update_example();
            format!("for_no_key_update_example")
        });

    let for_share_example = warp::get()
        .and(warp::path!("users" / "for_share"))
        .and(warp::path::end())
        .map(|| {
            user_model::for_share_example();
            format!("for_share_example")
        });
    let single_value = warp::get()
        .and(warp::path!("users" / "single_value"))
        .and(warp::path::end())
        .map(|| {
            user_model::single_value_example();
            format!("single_value_example")
        });
    let get_result_example = warp::get()
        .and(warp::path!("users" / "get_result"))
        .and(warp::path::end())
        .map(|| {
            user_model::get_result_example();
            format!("get_result_example")
        });

    let animal = warp::get()
        .and(warp::path!("animal" / "distinct-on"))
        .and(warp::path::end())
        .and_then(handlers::animal_handler::distinct_demo);

    let animal_or_filter = warp::get()
        .and(warp::path!("animal" / "or_filter"))
        .and(warp::path::end())
        .map(|| {
            animal_model::or_filter_example();
            format!("or_filter_example")
        });

    let animal_update_demo = warp::get()
        .and(warp::path!("animal" / "update"))
        .and(warp::path::end())
        .map(|| {
            animal_model::update_demo();
            format!("update")
        });

    use std::collections::HashMap;
    let get_demo = warp::get()
        .and(warp::path("get_demo"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("key") {
            Some(key) => warp::http::Response::builder().body(format!("key = {}", key)),
            None => {
                warp::http::Response::builder().body(String::from("No \"key\" param in query."))
            }
        });
    let get_example = warp::get()
        .and(warp::path("get_example"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(crate::handlers::get_demo_handler::get_example);
    let get_example_num = warp::get()
        .and(warp::path!("get_example" / "num"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, i64>>())
        .and_then(crate::handlers::get_demo_handler::get_example_number);
    let get_my = warp::get()
        .and(warp::path!("get_demo" / "my"))
        .and(warp::path::end())
        .and(warp::query::<crate::handlers::get_demo_handler::MyObject>())
        .and_then(crate::handlers::get_demo_handler::example);
    let opt_query = warp::query::<crate::handlers::get_demo_handler::MyObject>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<crate::handlers::get_demo_handler::MyObject>,), std::convert::Infallible>(
                (None,),
            )
        });
    let get_my_opt = warp::get()
        .and(warp::path!("get_demo" / "my_opt"))
        .and(warp::path::end())
        .and(opt_query)
        .and_then(crate::handlers::get_demo_handler::my_options);

    let routes = home
        .or(users)
        .or(user_select)
        .or(user_join)
        .or(user_count)
        .or(user_inner_join)
        .or(user_filter)
        .or(user_find_example)
        .or(user_order)
        .or(user_limit)
        .or(user_offset)
        .or(for_update_example)
        .or(no_wait_example)
        .or(skip_locked)
        .or(for_no_key_update)
        .or(for_share_example)
        .or(into_boxed_example)
        .or(single_value)
        .or(get_result_example)
        .or(animal)
        .or(animal_or_filter)
        .or(animal_update_demo)
        .or(get_demo)
        .or(get_example)
        .or(get_example_num)
        .or(get_my)
        .or(get_my_opt);
    routes
}
