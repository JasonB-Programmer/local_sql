use std::io::{self, BufRead};
use std::process::Command;

use async_std::net::TcpStream;
use tiberius::{Client, Config};
use once_cell::sync::Lazy;
use std::env;
use async_std::task;
use colored::*;


static CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=JASON\\SQLEXPRESS,61521;database=AdventureWorks2016_EXT;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
    })
});


/// Connect to an SQL Server instance using the hostname and port number.
pub async fn connect_through_port() -> anyhow::Result<()> {

    let config = Config::from_ado_string(&CONN_STR_PORT)?;
    
    // Create a `TCPStream` from the `async-std` library with 
    // a address that contains the hostname/IP and port number.
    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    // Connect to SQL Server
    let client = Client::connect(config, tcp).await?;

    // make this able to close remotely
    client.close().await?;

    Ok(())
}

fn main() {
    // Call the test function using block_on
    println!("{}", "\n Testing connection".yellow().bold());

    // Call the connection function using block_on
    match task::block_on(connect_through_port()) {

        Ok(_) => println!("{}", "Connection test successful!\n".green().bold()),
        Err(e) => panic!("Connection test failed: {}", e)
    }
    
    //promt message
    println!("{}", "Enter commands (type 'exit' to quit): \n".yellow().bold());
    println!("{}", "Enter 'commands' to see a list of commands \n".yellow().bold());
    
    //get user input
    let stdin = io::stdin();
    let handle = stdin.lock();


    //user input results
    for line in handle.lines() {
        let input = line.unwrap().trim().to_string();
        
        if input == "exit" {
            break;
        } else if input == "find tables" {
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(["/C", "start", "cmd", "/K", "cargo run --bin cargo-findtable"])
                    .spawn()
                    .expect("Failed to open new terminal");
            }

            #[cfg(target_os = "macos")]
            {
                Command::new("osascript")
                    .args(["-e", "tell application \"Terminal\" to do script \"cd $(pwd) && cargo run --bin read_table\""])
                    .spawn()
                    .expect("Failed to open new terminal");
            }

            #[cfg(target_os = "linux")]
            {
                Command::new("x-terminal-emulator")
                    .args(["-e", "cargo run --bin read_table"])
                    .spawn()
                    .expect("Failed to open new terminal");
            }
        } else if input == "commands" {
            println!("Commands:");
            println!("find tables - Find tables in the database");
            println!("read table - Read a table in the database");
            println!("exit - Exit the program");
            println!("commands - List all commands");

        } else if input == "read table" {
            #[cfg(target_os = "windows")]
            {
                // Open in a new command window and wait for completion
                let mut child = Command::new("cmd")
                    .args(["/C", "start", "cmd", "/C", "cargo run --bin cargo-readtable"])
                    .spawn()
                    .expect("Failed to start process");

                // Wait for the process to complete
                child.wait().expect("Failed to wait for process");
            }
        }
         else {
            println!("Unknown command: {}, use 'commands' to see a list of commands", input);
        }
    }
}

