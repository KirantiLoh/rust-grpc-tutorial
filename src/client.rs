use services::{
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient, PaymentRequest, TransactionRequest,
};
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

    let mut transaction_client = TransactionServiceClient::connect(addr).await?;

    let request = Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut stream = transaction_client
        .get_transaction_history(request)
        .await?
        .into_inner();

    while let Some(response) = stream.message().await? {
        println!("Response: {:?}", response);
    }

    Ok(())
}
