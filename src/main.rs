mod db;

use db::DB;
use tokio_postgres::Error;

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
