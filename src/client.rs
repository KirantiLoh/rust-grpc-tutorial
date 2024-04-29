use services::{payment_service_client::PaymentServiceClient, PaymentRequest};
use tonic::Request;

pub mod services {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::1]:50051";
    let mut client = PaymentServiceClient::connect(addr).await?;

    let request = Request::new(PaymentRequest {
        amount: 100.0,
        user_id: "user_123".to_string(),
    });

    let response = client.process_payment(request).await?;

    println!("Response: {:?}", response);

    Ok(())
}
