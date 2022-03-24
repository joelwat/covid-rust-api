use sea_orm::Statement;
use sea_schema::migration::{
    prelude::*,
    sea_orm::ConnectionTrait,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220323_201100_create_countries_regions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE countries_regions (
            id INT NOT NULL AUTO_INCREMENT,
            name VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        );"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE countries_regions;";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
