#[derive(Debug, Clone)]
pub struct TableColumnsBuilder;

impl TableColumnsBuilder {
    pub fn build(table: &str) -> String {
        format!("SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{table}'")
    }
}
