use models::websocket::websocket;
mod models;
mod market;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    websocket().await?;
    println!("connection Sucessefull!");
    Ok(())
   
}




