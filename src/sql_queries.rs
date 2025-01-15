use crate::sql_client as sc;

pub fn table() -> Result<(), Box<dyn std::error::Error>> {
    sc::read_table;
    Ok(())
}