use services::{
    chat_service_client::ChatServiceClient, payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient, ChatMessage, PaymentRequest,
    TransactionRequest,
};
use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Channel, Request};

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

    let channel = Channel::from_static(addr).connect().await?;

    let mut chat_client = ChatServiceClient::new(channel);

    let (tx, rx): (Sender<ChatMessage>, Receiver<ChatMessage>) = mpsc::channel(32);

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() {
                continue;
            }
            let message = ChatMessage {
                user_id: "user_123".to_string(),
                message: line,
            };
            if tx.send(message).await.is_err() {
                eprintln!("Failed to send message");
                break;
            }
        }
    });

    let request = tonic::Request::new(ReceiverStream::new(rx));
    let mut response = chat_client.chat(request).await?.into_inner();

    while let Some(message) = response.message().await? {
        println!("Received message: {:?}", message);
    }

    Ok(())
}
