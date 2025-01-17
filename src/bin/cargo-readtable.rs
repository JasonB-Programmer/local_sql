use tiberius_sqlserver::sql_client as sc;

fn main() {
    // Create a runtime for async operations
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    // Run the async function in the runtime
    runtime.block_on(async {
        match sc::read_table().await {
            Ok(_) => {
                println!("Successfully read table data!");
            },
            Err(e) => {
                eprintln!("Error reading table: {}", e);
                std::process::exit(1);
            }
        }
    });
}