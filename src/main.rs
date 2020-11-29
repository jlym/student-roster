mod db;

use actix_web::{get, web, App, HttpServer, Responder};
use db::DB;

/*
#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = DB::new("localhost", "roster", "postgres", "password").await?;
    let grade: i32 = 2;
    let student = db.add_student("Jeffrey", grade).await?;

    println!("student id: {}", student.id);
    println!("student name: {}", student.name);
    println!("student grade: {}", student.grade);
    Ok(())
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DB::new("localhost", "roster", "postgres", "password")
        .await
        .unwrap();
    HttpServer::new(move || App::new().data(db.clone()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/*
#[get("/")]
async fn index(_: web::Data<DB>) -> String {
    String::from("wow")
}
*/

#[get("/")]
async fn index(data: web::Data<DB>) -> String {
    let grade: i32 = 2;
    let student = data.add_student("Jeffrey", grade).await.unwrap();

    println!("student id: {}", student.id);
    println!("student name: {}", student.name);
    println!("student grade: {}", student.grade);

    student.name.clone()
}
