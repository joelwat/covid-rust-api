use sea_orm::Statement;
use sea_schema::migration::{
    prelude::*,
    sea_orm::ConnectionTrait,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220323_214600_create_days_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE days (
                id INT NOT NULL AUTO_INCREMENT,
                days_date DATE,
                last_update DATETIME,
                lat DECIMAL(20, 15),
                `long` DECIMAL(20, 15),
                confirmed INT,
                deaths INT,
                recovered INT,
                active INT,
                fips INT,
                incident_rate DECIMAL(25, 18),
                total_test_results INT,
                people_hospitalized INT,
                case_fatality_ratio DECIMAL(30, 20),
                uid INT,
                iso3 VARCHAR(255),
                testing_rate DECIMAL(25, 18),
                hospitalization_rate DECIMAL(25, 18),

                PRIMARY KEY (id)
            );
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE days;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
