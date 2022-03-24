pub use sea_schema::migration::*;

mod m20220323_201100_create_countries_regions_table;
mod m20220323_213600_create_provinces_states_table;
mod m20220323_214600_create_days_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220323_201100_create_countries_regions_table::Migration),
            Box::new(m20220323_213600_create_provinces_states_table::Migration),
            Box::new(m20220323_214600_create_days_table::Migration),
        ]
    }
}
