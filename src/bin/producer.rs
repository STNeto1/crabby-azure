use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok().expect("failed to load .env");

    let payload = crabby_azure::Payload {
        message: "my first message".to_string(),
    };
    let serialized = payload.to_string().unwrap(); // should handle better

    let client = crabby_azure::create_azure_client();

    client.send_message(&serialized.as_str()).await.unwrap();
}
