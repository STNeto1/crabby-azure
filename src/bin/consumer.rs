use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok().expect("failed to load .env");

    let client = crabby_azure::create_azure_client();

    let received_message = client.receive_and_delete_message().await.unwrap();

    let payload = crabby_azure::Payload::from_string(&received_message).unwrap();

    println!("{:?}", payload);
}
