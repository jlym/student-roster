use tokio_postgres::{Client, Config, Error, NoTls};

pub struct DB {
    pub client: Client,
}

pub struct Student {
    pub id: i32,
    pub name: String,
    pub grade: i32,
}

impl DB {
    pub async fn new(host: &str, dbname: &str, user: &str, password: &str) -> Result<Self, Error> {
        let mut config = Config::new();
        let config = config
            .host(host)
            .dbname(dbname)
            .user(user)
            .password(password);
        let (client, connection) = config.connect(NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection: error: {}", e);
            }
        });

        let db = Self { client };
        Ok(db)
    }

    pub async fn add_student(&self, name: &str, grade: i32) -> Result<Student, Error> {
        let rows = self
            .client
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
