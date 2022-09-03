use crate::db;
use crate::schema::animals;
use crate::schema::animals::dsl::*;
use diesel::prelude::*;

#[derive(Debug, Queryable, PartialEq)]
pub struct Animal {
    species: String,
    name: Option<String>,
    legs: i32,
}

impl Animal {
    fn new<S: Into<String>>(special: S, named: Option<&str>, leg: i32) -> Self {
        Animal {
            species: special.into(),
            name: named.map(Into::into),
            legs: leg,
        }
    }
}

pub fn or_filter_example() {
    let connection = db::establish_connection();
    diesel::delete(animals).execute(&connection).unwrap(); //删除表数据
    diesel::insert_into(animals)
        .values(&vec![
            (species.eq("cat"), legs.eq(4), name.eq("Sinatra")),
            (species.eq("dog"), legs.eq(3), name.eq("Fido")),
            (species.eq("spider"), legs.eq(8), name.eq("Charlotte")),
        ])
        .execute(&connection)
        .unwrap();

    let good_animals = animals
        .filter(name.eq("Fido"))
        .or_filter(legs.eq(4))
        .select(name)
        .get_results::<Option<String>>(&connection);
    println!("or_filter_example:{:?}", good_animals);

    type DB = diesel::pg::Pg;
    let sql = diesel::debug_query::<DB, _>(
        &animals
            .filter(name.eq("Fido"))
            .or_filter(legs.eq(4))
            .select(name),
    )
    .to_string();
    println!("SQL:{:?}", sql);
    println!("or_filtter_example end");
}

#[derive(Queryable, Debug, PartialEq)]
struct AnimalQuery {
    id: i32,
    species: String,
    legs: i32,
    name: Option<String>,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "animals"]
struct AnimalForm<'a> {
    id: i32,
    name: &'a str,
}

pub fn run_test() {
    let connection = db::establish_connection();
    let form = AnimalForm {
        id: 2,
        name: "Super scary",
    };
    // let changed_animal=form.save_changes(&connection);
}

#[derive(AsChangeset)]
#[table_name = "animals"]
pub struct UpdateAnimal<'a> {
    pub legs: i32,
    pub name: &'a str,
}

// 用结构体更新表数据示例更新示例
pub fn update_demo() {
    let connection = db::establish_connection();
    let update = UpdateAnimal {
        legs: 34,
        name: "修改后的名字",  //&form.title[..],
    };

    let changed_data = diesel::update(animals.find(3))
        .set(&update)
        .get_result::<AnimalQuery>(&connection)
        .expect("k");
    //无数据：thread 'tokio-runtime-worker' panicked at 'k: NotFound', src/models/animal_model.rs:96:10
}
