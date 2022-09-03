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

pub fn find_example() {
    use diesel::result::Error::NotFound;

    let sean = (1, "Sean".to_string());
    let tess = (2, "Tess".to_string());

    let connection = db::establish_connection();
    assert_eq!(Ok(sean), users.find(1).first(&connection));
    assert_eq!(Ok(tess), users.find(2).first(&connection));

    assert_eq!(
        Err::<(i32, String), _>(NotFound),
        users.find(3).first(&connection)
    );

    println!("find example");

    type DB = diesel::pg::Pg;
    let sql = diesel::debug_query::<DB, _>(&users.find(3)).to_string();
    println!("SQL:{:?}", sql);
}

pub fn order_example() {
    let connection = db::establish_connection();
    connection.execute("DELETE FROM users").unwrap();
    diesel::insert_into(users)
        .values(&vec![name.eq("Saul"), name.eq("Steve"), name.eq("Stan")])
        .execute(&connection)
        .unwrap();

    let ordered_names = users
        .select(name)
        .order(name.desc())
        .load::<String>(&connection)
        .unwrap();
    println!("order example: {:?}", ordered_names);

    diesel::insert_into(users)
        .values(name.eq("Stan"))
        .execute(&connection)
        .unwrap();

    let data = users
        .select((name, id))
        .order((name.asc(), id.desc()))
        .load::<(String, i32)>(&connection)
        .unwrap();

    println!("asc desc:{:?}", data);
    println!("order example end");

    let data = users
        .select((name, id))
        .order_by(name.asc())
        .then_order_by(id.desc())
        .load::<(String, i32)>(&connection)
        .unwrap();
    println!("then_order_by:{:?}", data);
}

pub fn limit_example() {
    let connection = db::establish_connection();
    diesel::delete(users).execute(&connection).unwrap();
    diesel::insert_into(users)
        .values(&vec![
            name.eq("Sean"),
            name.eq("Bastien"),
            name.eq("Pascal"),
        ])
        .execute(&connection)
        .unwrap();

    let limited = users
        .select(name)
        .order(id)
        .limit(1)
        .load::<String>(&connection)
        .unwrap();
    println!("limit data:{:?}", limited);

    let no_limit = users
        .select(name)
        .order(id)
        .load::<String>(&connection)
        .unwrap();
    println!("no limit data:{:?}", no_limit);

    println!("limit example end");
}

pub fn offset_example() {
    let connection = db::establish_connection();
    diesel::delete(users).execute(&connection).unwrap();
    diesel::insert_into(users)
        .values(&vec![
            name.eq("Sean"),
            name.eq("Bastien"),
            name.eq("Pascal"),
        ])
        .execute(&connection)
        .unwrap();

    let query = users.select(name).order(id).limit(2);

    type DB = diesel::pg::Pg;

    let offset = query.offset(1).load::<String>(&connection).unwrap();
    println!("offset data:{:?}", offset);
    let sql = debug_query::<DB, _>(&query.offset(1)).to_string();
    println!("SQL:{:?}", sql);

    let no_offset = query.load::<String>(&connection).unwrap();
    println!("no offset data:{:?}", no_offset);
    let sql = debug_query::<DB, _>(&query).to_string();
    println!("SQL:{:?}", sql);

    println!("offset example end");
}

pub fn skip_locked_example() {
    let connection = db::establish_connection();

    let query = users.for_update().skip_locked();
    let data = query.load::<(i32, String)>(&connection).unwrap();
    println!("data:{:?}", data);

    println!("SQL:{:?}", diesel::debug_query(&query).to_string());
    println!("FOR UPDATE SKIP LOCKED end");
}

pub fn no_wait_example() {
    let connection = db::establish_connection();
    let query = users.for_update().no_wait();

    let data = query.load::<(i32, String)>(&connection).unwrap();
    println!("data:{:?}", data);

    println!("SQL:{:?}", diesel::debug_query(&query).to_string());
    //SELECT \"users\".\"id\", \"users\".\"name\" FROM \"users\" FOR UPDATE NOWAIT
    println!("FOR UPDATE NOWAIT end");
}

/// 添加行锁，不能与（distinct,group by,unions等）使用。
/// 得添加with-deprecated才能使用for_update,例：diesel = { version="1.4.6",features=["extras","postgres","r2d2","with-deprecated"] }
pub fn for_update_example() {
    let connection = db::establish_connection();
    type DB = diesel::pg::Pg;

    // users.select(name).for_update.load::<String>(&connection);
    // SELECT \"users\".\"name\" FROM \"users\" FOR UPDATE
    let query = users.select(name).for_update();
    // let sql = diesel::debug_query::<DB, _>(&query).to_string();
    let sql = diesel::debug_query(&query).to_string();
    println!("悲观锁SQL：{:?}", sql);
    let data = query.load::<String>(&connection).unwrap();
    println!("data:{:?}", data);

    println!("for update悲观锁 end");
}

/// postgreSQL专有 for_no_key_update
pub fn for_no_key_update_example() {
    let connection = db::establish_connection();

    let query = users.for_no_key_update();
    let data = query.load::<(i32, String)>(&connection);
    println!("FOR NO KEY UPDATE :{:?}", data);
    println!("SQL:{:?}", diesel::debug_query(&query).to_string());
    //SELECT \"users\".\"id\", \"users\".\"name\" FROM \"users\" FOR NO KEY UPDATE

    println!("FOR NO KEY UPDATE end");
}

/// postgreSQL专有FOR SHARE
pub fn for_share_example() {
    let connection = db::establish_connection();

    let query = users.for_share();
    let data = query.load::<(i32, String)>(&connection).unwrap();
    println!("FOR SHARE data:{:?}", data);
    println!("SQL:{:?}", diesel::debug_query(&query).to_string());
    //SELECT \"users\".\"id\", \"users\".\"name\" FROM \"users\" FOR SHARE

    println!("FOR SHARE end");

    let query = users.for_key_share();
    let data = query.load::<(i32, String)>(&connection).unwrap();
    println!("FOR KEY SHARE data:{:?}", data);
    println!("SQL:{:?}", diesel::debug_query(&query).to_string());
    //SELECT \"users\".\"id\", \"users\".\"name\" FROM \"users\" FOR KEY SHARE

    println!("FOR KEY SHARE end");
}

/// 应用场景在后台搜索条件查询时，连接条件
pub fn into_boxed_example() {
    let connection = db::establish_connection();

    use std::collections::HashMap;
    let mut params = HashMap::new();
    params.insert("name", "Sean");

    let mut query = users::table.into_boxed();
    if let Some(nam) = params.get("name") {
        query = query.filter(users::name.eq(nam));
    }

    println!("SQL:{:?}", diesel::debug_query(&query).to_string());
    let data = query.load::<(i32, String)>(&connection);
    println!("data:{:?}", data);

    println!("into_boxed end");
}

/// `foo = (SELECT ...)`这样子的查询语句
pub fn single_value_example() {
    use crate::schema::posts;
    let connection = db::establish_connection();

    let last_post = posts::table.order(posts::id.desc());

    let most_recently_active_user = users
        .select(name)
        .filter(
            id.nullable()
                .eq(last_post.select(posts::user_id).single_value()),
        )
        .first::<String>(&connection)
        .unwrap();
    println!("most_recently_active_user:{:?}", most_recently_active_user);

    let query = users.select(name).filter(
        id.nullable()
            .eq(last_post.select(posts::user_id).single_value()),
    );
    let data = query.first::<String>(&connection).unwrap();
    println!("data:{:?}", data);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    println!("SQL:{:?}", sql);

    println!("single_value end");
}

pub fn get_result_example() {
    let connection = db::establish_connection();
    let inserted_row = diesel::insert_into(users)
        .values(name.eq("Ruby"))
        .get_result::<User>(&connection)
        .unwrap();
    println!("inserted_row: {:?}", inserted_row);

    let update_result = diesel::update(users.find(31))
        .set(name.eq("Jim"))
        .get_result::<(i32, String)>(&connection);
    println!("update_result:{:?}", update_result);
    println!("get_result end");
}
