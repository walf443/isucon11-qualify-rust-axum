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

    pub async fn prepare_tables(&mut self, tables: Vec<String>) -> Result<(), sqlx::Error> {
        for table in tables {
            self.prepare_table(&table).await?;
        }

        Ok(())
    }

    async fn truncate_table(&self, table: &str) -> Result<(), sqlx::Error> {
        // check table name string
        let sql = format!("TRUNCATE TABLE `{}`", table);
        sqlx::query(&sql).execute(&self.pool).await?;

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
            // no panic for test assert error
            error!("please call clean before drop");
        }
    }
}
