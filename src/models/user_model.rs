use crate::db;
use crate::schema::users::dsl::*;
use crate::schema::{posts, users};
use diesel::debug_query;
use diesel::prelude::*;

#[derive(Debug, Queryable, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,
}
impl User {
    fn new(uid: i32, uname: &str) -> Self {
        User {
            id: uid,
            name: uname.into(),
        }
    }
}

#[derive(Debug, Queryable, PartialEq, Eq)]
pub struct Post {
    id: i32,
    user_id: i32,
    title: String,
}
impl Post {
    fn new(pid: i32, userid: i32, ptitle: &str) -> Self {
        Post {
            id: pid,
            user_id: userid,
            title: ptitle.into(),
        }
    }
}

/// `DISTINCT`关键字查询
pub fn distinct_test() {
    println!("run_test");
    let connection = db::establish_connection();
    connection.execute("DELETE FROM users").unwrap(); //删除表数据

    /* 插入表数据 */
    let result = diesel::insert_into(users)
        .values(&vec![name.eq("Sean"); 3])
        .execute(&connection)
        .map_err(|e| format!("插入数据出错:{}", e));

    let names = users.select(name).load::<String>(&connection);
    let distinct_names = users.select(name).distinct().load::<String>(&connection);
    println!("name:{:?}", names); //name:Ok(["Sean", "Sean", "Sean"])
                                  // assert_eq!(vec!["Sean"; 3], names);

    println!("distinct_names:{:?}", distinct_names); //distinct_names:Ok(["Sean"])

    // assert_eq!(vec!["Sean"; 1], distinct_names);
}

pub fn demo_select() {
    let connection = db::establish_connection();

    let all_users = users.load::<(i32, String)>(&connection);
    match all_users {
        Ok(allusers) => println!("users表所有数据：{:?}", allusers),
        Err(e) => println!("users表查无数据:{:?}", e),
    }

    let all_names = users.select(name).load::<String>(&connection);
    match all_names {
        Ok(allnames) => println!("所有名字： {:?}", allnames),
        Err(e) => println!("查无数据：{:?}", e),
    }
}

pub fn demo_join() {
    let connection = db::establish_connection();
    /* 联表查询 */
    let result = connection.execute("DELETE FROM posts"); //删除表posts数据
    diesel::insert_into(posts::table)
        .values((posts::user_id.eq(10), posts::title.eq("Sean's Post")))
        .execute(&connection)
        .unwrap();

    let postID = posts::table.select(posts::id).first::<i32>(&connection);

    /* 要用join得在schema.rs中添加“joinable!(posts->users(user_id));”，不然会报错 */
    let join = users::table.left_join(posts::table);
    let all_data = join.load::<(User, Option<Post>)>(&connection);
    match all_data {
        Ok(data) => println!("联表查询数据：{:?}", data),
        Err(e) => println!("联表查无数据：{:?}", e),
    }

    let names_and_titles = join
        .select((users::name, posts::title.nullable()))
        .load::<(String, Option<String>)>(&connection);

    match names_and_titles {
        Ok(namesandtitles) => println!("联表数据：{:?}", namesandtitles),
        Err(e) => println!("联表查无数据：{:?}", e),
    }
    // print_sql!();
}

pub fn demo_count() {
    let connection = db::establish_connection();

    let count_user: i64 = users
        .count()
        .get_result(&connection)
        .expect("统计users表出错");
    println!("users表总共有{}条数据", count_user);

    /* 输出SQL */
    type DB = diesel::pg::Pg;
    let sql = debug_query::<DB, _>(&users.count()).to_string();
    println!("SQL:{:?}", sql);

    let query = users.find(10);
    let debug = debug_query::<DB, _>(&query);
    println!("SQL: {:?}", debug);

    let all_names = users.select(name);
    let debug = debug_query::<DB, _>(&all_names);
    println!("SQL: {:?}", debug);
}

pub fn inner_join_example() {
    /*
    联表查询记得在schema.rs文件中添加如下两行
    joinable!(posts -> users (user_id));
    allow_tables_to_appear_in_same_query!(users, posts);
    */
    println!("inner_join示例 start");

    use crate::schema::posts::dsl::{posts, title, user_id};

    let connection = db::establish_connection();

    let data = users
        .inner_join(posts)
        .select((name, title))
        .load::<(String, String)>(&connection);
    match data {
        Ok(join_data) => println!("join data: {:?}", join_data),
        Err(e) => println!("联表查无数据:{:?}", e),
    }

    /* 如此拆分可以输出调试的SQL */
    let users_join_posts = users.inner_join(posts).select((name, title));
    let data = users_join_posts.load::<(String, String)>(&connection);
    match data {
        Ok(join_data) => println!("join data: {:?}", join_data),
        Err(e) => println!("联表查无数据:{:?}", e),
    }
    type DB = diesel::pg::Pg;
    let sql = debug_query::<DB, _>(&users_join_posts).to_string();
    println!("SQL:{:?}", sql);

    /*
    schema.rs文件中不添加joinable!时要显示用ON来连接查询
    */
    diesel::insert_into(posts)
        .values(&vec![
            (user_id.eq(1), title.eq("Sean's post")),
            (user_id.eq(8), title.eq("Sean is a jerk")),
        ])
        .execute(&connection)
        .unwrap();

    let data = users
        .inner_join(posts.on(title.like(name.concat("%"))))
        .select((name, title))
        .load::<(String, String)>(&connection);
    match data {
        Ok(join_data) => println!("join data: {:?}", join_data),
        Err(e) => println!("联表查无数据:{:?}", e),
    }

    let ujp = users
        .inner_join(posts.on(title.like(name.concat("%"))))
        .select((name, title));
    let data = ujp.load::<(String, String)>(&connection);
    match data {
        Ok(join_data) => println!("join data: {:?}", join_data),
        Err(e) => println!("联表查无数据:{:?}", e),
    }
    let sql = debug_query::<DB, _>(&ujp).to_string();
    println!("SQL:{:?}", sql);

    println!("inner_join示例 end");
}

pub fn filter_example() {
    let connection = db::establish_connection();

    let seans_id = users
        .filter(name.eq("Sean"))
        .select(id)
        .first::<i32>(&connection);

    println!("filter_example:{:?}", seans_id);

    type DB = diesel::pg::Pg;
    let sql = debug_query::<DB, _>(&users.filter(name.eq("Sean"))).to_string();
    println!("SQL:{:?}", sql);

    let tess_id = users
        .filter(name.eq("Tess"))
        .select(id)
        .first::<i32>(&connection);
    println!("filter_example:{:?}", tess_id);
}
