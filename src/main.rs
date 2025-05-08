use message_svc::pb::{
    SendEmailRequest, SendEmailResponse, SendSmsRequest, SendSmsResponse, message_server::Message,
    message_server::MessageServer,
};
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message_svc = MessageService {};

    tonic::transport::Server::builder()
        .add_service(MessageServer::new(message_svc))
        .serve("[::1]:50053".parse()?)
        .await?;

    Ok(())
}

pub struct MessageService {}

#[tonic::async_trait]
impl Message for MessageService {
    async fn send_sms(
        &self,
        req: Request<SendSmsRequest>,
    ) -> Result<Response<SendSmsResponse>, Status> {
        todo!()
    }

    async fn send_email(
        &self,
        req: Request<SendEmailRequest>,
    ) -> Result<Response<SendEmailResponse>, Status> {
        todo!()
    }
}
