use deadpool_postgres::config::Config;
use deadpool_postgres::Pool;
use tokio_postgres::{Error, NoTls};

#[derive(Clone)]
pub struct DB {
    pub pool: Pool,
}

pub struct Student {
    pub id: i32,
    pub name: String,
    pub grade: i32,
}

impl DB {
    pub async fn new(host: &str, dbname: &str, user: &str, password: &str) -> Result<Self, Error> {
        let mut config = Config::new();
        config.host = Some(String::from(host));
        config.dbname = Some(String::from(dbname));
        config.user = Some(String::from(user));
        config.password = Some(String::from(password));

        let pool = config.create_pool(NoTls).unwrap();

        let db = Self { pool };
        Ok(db)
    }

    pub async fn add_student(&self, name: &str, grade: i32) -> Result<Student, Error> {
        let client = self.pool.get().await.unwrap();
        let rows = client
            .query(
                "INSERT INTO students (name, grade)
                VALUES($1, $2)
                RETURNING student_id, name, grade;",
                &[&name, &grade],
            )
            .await?;

        let id: i32 = rows[0].get("student_id");
        let name: String = rows[0].get("name");
        let grade: i32 = rows[0].get("grade");
        let student = Student { id, name, grade };

        Ok(student)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_student_works() -> Result<(), Error> {
        let db = new_test_db().await?;
        let student = db.add_student("Fred", 5).await?;

        assert_eq!(student.name, "Fred");
        assert_eq!(student.grade, 5);
        assert!(student.id > 0);
        Ok(())
    }

    async fn new_test_db() -> Result<DB, Error> {
        DB::new("localhost", "roster", "postgres", "password").await
    }
}
