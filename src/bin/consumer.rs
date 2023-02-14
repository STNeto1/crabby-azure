use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok().expect("failed to load .env");

    let client = crabby_azure::create_azure_client();

    loop {
        match client.receive_and_delete_message().await {
            Ok(msg) => {
                if let Ok(payload) = crabby_azure::Payload::from_string(&msg) {
                    println!("received => {:?}", payload);
                } else {
                    println!("failed to decode payload => '{}'", msg);
                }
            }
            Err(e) => println!("error => {}", e),
        }
    }
}
