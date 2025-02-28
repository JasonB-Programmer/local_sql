use tiberius_sqlserver::sql_client as sc;
use std::io::{self, Write};
use std::process::Command;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    // Run the async function in the runtime
    runtime.block_on(async {
        match sc::find_table_all().await {
            Ok(_) => {
                println!("Successfully found table data!");
            },
            Err(e) => {
                eprintln!("Error reading table: {}", e);
                std::process::exit(1);
            }
        }
    });

    println!("\nPress Enter to close...");
    io::stdout().flush()?;

    // Wait specifically for Enter key
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        // Check if the input is just a newline character
        if input.trim().is_empty() {
            #[cfg(target_os = "windows")]
            {
                std::process::exit(0);
            }

            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                std::process::exit(0);
            }
        } else {
            println!("Please press Enter to close...");
            io::stdout().flush()?;
        }
    }
}
