#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

use rocket_contrib::json::Json;
use serde_json::{Value as JsonValue};

use rocket::http::RawStr;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/nama/<name>/<umur>")]
fn nama_user(name: &RawStr, umur: i32) -> String {
    let pak_atau_mas = if umur > 30 { "pak" } else { "mas" };

    format!("Hello, {} {}!", pak_atau_mas, name.as_str())
}

#[get("/download/<file..>")]
fn ke_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(Serialize, Deserialize)]
struct Anggota {
    pub nama: String,
    pub email: Option<String>,
}

#[derive(Serialize)]
struct ApiResult {
    pub result: Vec<Anggota>,
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DB:Arc<Mutex<HashMap<String, String>>> = {
        let d = HashMap::new();
        // d.insert("muiz".to_string(), r#"{"nama":"Muiz","email":"muiz@gmail.com"}"#.to_string());
        Arc::new(Mutex::new(d))
    };
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(data: Json<Anggota>) -> Json<JsonValue> {
    // println!("{:?}", *DB);

    if data.email.is_none(){
        return Json(json!({"error": "email kosong, perlu diisi ndul!"}));
    }

    let mut db = DB.lock().unwrap();

    db.insert(data.nama.to_owned(), data.email.as_ref().unwrap().to_owned());

    dbg!(&*db);

    // Json(ApiResult {
    //     result: vec![data.into_inner()],
    // })
    Json(
        serde_json::value::to_value(
            ApiResult {
                result: vec![data.into_inner()]
            }
        ).expect("Gagal meng-serialize data")
    )
}

#[get("/anggota")]
fn daftar_anggota() -> Json<ApiResult> {
    let db = DB.lock().unwrap();

    let daftar = db
        .iter()
        .map(|(k, v)| Anggota {
            nama: k.to_owned(),
            email: Some(v.to_owned()),
        })
        .collect();

    Json(ApiResult { result: daftar })
}

#[post("/anggota/update", data = "<data>")]
fn update(data: Json<Anggota>) -> Json<ApiResult> {
    let mut db = DB.lock().unwrap();
    let mut data_baru: Option<Anggota> = None;

    match db.iter_mut().find(|(k, _v)| *k == &data.nama) {
        Some((k, v)) => {
            *v = data.email.as_ref().unwrap().to_owned();
            data_baru = Some(Anggota {
                nama: k.to_owned(),
                email: Some(v.to_owned()),
            });
        }
        None => (),
    }

    Json(ApiResult {
        result: data_baru.into_iter().collect(),
    })
}

#[post("/anggota/delete", data = "<data>")]
fn delete(data: Json<Anggota>) -> String {
    
    let mut db = DB.lock().unwrap();

    db.remove(&data.nama);

    format!("Akun anggota `{}` telah dihapus.", data.nama)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, nama_user, ke_file, register, 
                daftar_anggota, update, delete],
        )
        .launch();
}
