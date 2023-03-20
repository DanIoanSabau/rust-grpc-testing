use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BitcoinPaymentRequest, BitcoinPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService;

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(&self, request: Request<BitcoinPaymentRequest>) -> Result<Response<BitcoinPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request: BitcoinPaymentRequest = request.into_inner();
        let response = BitcoinPaymentResponse {
            successful: true,
            message: format!("sending {}BTC to {}", request.amount, request.to_addr)
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let bitcoin_serice = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(bitcoin_serice))
        .serve(addr)
        .await?;

    Ok(())
}
