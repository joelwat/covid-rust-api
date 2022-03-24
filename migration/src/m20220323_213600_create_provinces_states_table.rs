use sea_orm::Statement;
use sea_schema::migration::{
    prelude::*,
    sea_orm::ConnectionTrait,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220323_213600_create_provinces_states_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE provinces_states (
                id INT NOT NULL AUTO_INCREMENT,
                name VARCHAR(255) NOT NULL,
                country_region_id INT NOT NULL,
                PRIMARY KEY (id),

                FOREIGN KEY (country_region_id)
                REFERENCES countries_regions(id)
                ON UPDATE CASCADE
                ON DELETE CASCADE
            );
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE provinces_states;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
