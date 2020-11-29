mod db;

use actix_web::{get, web, App, HttpServer};
use db::DB;

#[actix_web::main]
async fn main() {
    let result = run().await;
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let db = DB::new("localhost", "roster", "postgres", "password").await?;
    HttpServer::new(move || App::new().data(db.clone()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index(data: web::Data<DB>) -> String {
    let grade: i32 = 2;
    let student = data.add_student("Jeffrey", grade).await.unwrap();

    println!("student id: {}", student.id);
    println!("student name: {}", student.name);
    println!("student grade: {}", student.grade);

    student.name.clone()
}
