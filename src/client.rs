use payments::bitcoin_client::BitcoinClient;
use payments::BitcoinPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = BitcoinPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "654321".to_owned(),
        amount: 22,
    };
    let request = tonic::Request::new(request);
    let response = client.send_payment(request).await?;

    println!("response: {:?}", response);

    Ok(())
}