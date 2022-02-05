use sqlx::MySqlPool;

pub struct Cleaner {
    pool: sqlx::MySqlPool,
    target_tables: Vec<String>,
    destroyed: bool,
}

impl Cleaner {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            pool,
            target_tables: Vec::new(),
            destroyed: false,
        }
    }

    pub async fn prepare_table(&mut self, table: &str) -> Result<(), sqlx::Error> {
        let _ = &self.target_tables.push(table.into());

        // if it was occured a panic when previous test, Cleaner can't clean table.
        self.truncate_table(table).await?;

        Ok(())
    }

    pub async fn truncate_table(&self, table: &str) -> Result<(), sqlx::Error> {
        sqlx::query("truncate table user")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn clean(&mut self) -> Result<(), sqlx::Error> {
        for table in &self.target_tables {
            self.truncate_table(table).await?;
        }

        self.destroyed = true;

        Ok(())
    }
}

impl Drop for Cleaner {
    fn drop(&mut self) {
        if !self.destroyed {
            panic!("you must call clean before drop!!");
        }
    }
}
