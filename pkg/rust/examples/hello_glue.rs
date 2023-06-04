#[cfg(feature = "sled-storage")]
mod api_usage {
    use {
        futures::executor::block_on,
        gluesql::prelude::{Glue, SledStorage},
    };

    fn mutable_api() {
        let storage = SledStorage::new("data/mutable-api").unwrap();
        let mut glue = Glue::new(storage);

        let sqls = [
            "CREATE TABLE Glue (id INTEGER, name TEXT, greeting TEXT);",
            "INSERT INTO Glue VALUES (1, 'Minwoo', 'Hello~');",
            "INSERT INTO Glue VALUES (2, 'OSS', 'Hi!');",
            "UPDATE Glue SET name = 'ProjectGlue' WHERE id=2;",
            "SELECT * FROM Glue;",
            "DELETE FROM Glue WHERE id=1;",
            "SELECT * FROM Glue;",
            "DROP TABLE Glue;"
        ];

        for sql in sqls {
            glue.execute(sql).unwrap();
        }
    }

    pub fn run() {
        mutable_api();
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    api_usage::run();
}
