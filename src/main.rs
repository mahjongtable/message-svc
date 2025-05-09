use std::{env, rc::Rc, sync::Arc};

use arc_swap::ArcSwap;
use lettre::{
    Message as EmailMessage, SmtpTransport, Transport,
    message::{MaybeString, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use message_svc::{
    pb::{
        SendEmailRequest, SendEmailResponse, SendSmsRequest, SendSmsResponse,
        message_server::{Message, MessageServer},
    },
    settings::{self, AppSettings},
};
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init and load app settings
    let app_settings = settings::AppSettings::new("settings.toml")?;

    let message_svc = MessageService {
        settings: ArcSwap::from(Arc::new(app_settings)),
    };

    tonic::transport::Server::builder()
        .add_service(MessageServer::new(message_svc))
        .serve("[::1]:50053".parse()?)
        .await?;

    Ok(())
}

pub struct MessageService {
    settings: ArcSwap<AppSettings>,
}

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
        // get mail config
        let mail_cfg = &self.settings.load().mail;

        // sender values
        let from_value = format!("{} <{}>", mail_cfg.from_name, mail_cfg.from_address)
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
            .body(req.get_ref().message_text.clone())
            .unwrap();

        let credentials = Credentials::new(mail_cfg.username.clone(), mail_cfg.password.clone());

        let mailer = SmtpTransport::relay(&mail_cfg.host)
            .map_err(|err| Status::internal(&err.to_string()))?
            .credentials(credentials)
            .build();

        match mailer.send(&message) {
            Ok(_) => {
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
    }
}
