use std::env;

use lettre::{
    Message as EmailMessage, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use message_svc::pb::{
    SendEmailRequest, SendEmailResponse, SendSmsRequest, SendSmsResponse,
    message_server::{Message, MessageServer},
};
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init env
    dotenvy::dotenv().expect(".env file doesn't exist");

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
        // env values
        let mail_host = env::var("MAIL_HOST")
            .map_err(|_| Status::internal("MAIL_HOST in the env file doesn't exist"))?;
        let mail_from_name = env::var("MAIL_FROM_NAME")
            .map_err(|_| Status::internal("MAIL_FROM_NAME in the env file doesn't exist"))?;
        let mail_from_address = env::var("MAIL_FROM_ADDRESS")
            .map_err(|_| Status::internal("MAIL_FROM_ADDRESS in the env file doesn't exist"))?;
        let mail_username = env::var("MAIL_USERNAME")
            .map_err(|_| Status::internal("MAIL_USERNAME in the env file doesn't exist"))?;
        let mail_password = env::var("MAIL_PASSWORD")
            .map_err(|_| Status::internal("MAIL_PASSWORD in the env file doesn't exist"))?;

        // sender values
        let from_value = format!("{} <{}>", mail_from_name, mail_from_address)
            .parse()
            .map_err(|_| Status::internal("invalid 'From' email value"))?;

        let to_value = format!("{} <{}>", "TBD", req.get_ref().email_address)
            .parse()
            .map_err(|_| Status::internal("invalid 'To' email value"))?;

        let message = EmailMessage::builder()
            .from(from_value)
            // .reply_to("Yuin <yuin@domain@tld>".parse().unwrap())
            .to(to_value)
            .subject("TBD") // TODO: Add subject request value
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be Happy"))
            .unwrap();

        let credentials = Credentials::new(
            mail_username,
            mail_password,
        );

        let mailer = SmtpTransport::relay(&mail_host)
            .unwrap()
            .credentials(credentials)
            .build();

        match mailer.send(&message) {
            Ok(o_o) => {
                return Ok(Response::new(SendEmailResponse {
                    status: true,
                    message: "sent successfully".to_string(),
                }));
            }
            Err(err) => {
                let err_msg = format!("could't send email: {err:?}");
                return Err(Status::internal(err_msg));
            }
        }

        // Ok(Response::new(SendEmailResponse {
        //     status: true,
        //     message: "ok".to_string(),
        // }))
    }
}
