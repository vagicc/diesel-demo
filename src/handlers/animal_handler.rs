use crate::db;
use crate::models::animal_model;
use crate::schema::animals::dsl::*;
use crate::schema::animals;
use diesel::prelude::*;

pub async fn distinct_demo() -> std::result::Result<impl warp::Reply, warp::Rejection> {
    println!("' DISTINCT ON '子句查询");

    let connection = db::establish_connection();
    connection.execute("DELETE FROM animals").unwrap(); //删除animals表数据
    let result = diesel::insert_into(animals)
        .values(&vec![
            (species.eq("dog"), name.eq(Some("招财狗")), legs.eq(4)),
            (species.eq("dog"), name.eq(None), legs.eq(4)),
            (species.eq("spider"), name.eq(None), legs.eq(8)),
        ])
        .execute(&connection)
        .unwrap();

    // let all_animals = animals.select((species, name, legs)).load(&connection);
    let all_animals = animals
        .select((species, name, legs))
        .load::<animal_model::Animal>(&connection);

    match all_animals {
        Ok(kk) => println!("所有的:{:?}", kk),
        Err(e) => println!("没有数据:{}", e),
    }

    let distinct_animals = animals
        .select((species, name, legs))
        .distinct_on(species)
        .load::<animal_model::Animal>(&connection);
    // let distinct_animals = animals.select((species, name, legs)).distinct_on(species).load(&connection);
    match distinct_animals {
        Ok(d_animals) => println!("distinct_on结果：{:?}", d_animals),
        Err(e) => println!("没有数据:{}", e),
    }

    let html = "`DISTINCT ON`子句查询".to_string();
    Ok(warp::reply::html(html))
}
