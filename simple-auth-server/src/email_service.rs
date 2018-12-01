use models::Invitation;
use sparkpost::transmission::{
    EmailAddress, Message, Options, Recipient, Transmission, TransmissionResponse,
};

fn get_api_key() -> String {
    std::env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set")
}

#[allow(unused)]
pub fn send_invitation(invitation: &Invitation) {
    let tm = Transmission::new_eu(get_api_key());
    let sending_email =
        std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
    // new email message with sender name and email
    let mut email = Message::new(EmailAddress::new(sending_email, "Let's Organise"));

    let options = Options {
        open_tracking: false,
        click_tracking: false,
        transactional: true,
        sandbox: false,
        inline_css: false,
        start_time: None,
    };

    // recipient from the invitation email
    let recipient: Recipient = invitation.email.as_str().into();

    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:3000/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>
         your Invitation expires on <strong>{}</strong>",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );


    // complete the email message with details
    email
        .add_recipient(recipient)
        .options(options)
        .subject("You have been invited to join Simple-Auth-Server Rust")
        .html(email_body);

    let result = tm.send(&email);

    match result {
        Ok(res) => {
            // println!("{:?}", &res);
            match res {
                TransmissionResponse::ApiResponse(api_res) => {
                    println!("API Response: \n {:#?}", api_res);
                    //   assert_eq!(1, api_res.total_accepted_recipients);
                    //   assert_eq!(0, api_res.total_rejected_recipients);
                }
                TransmissionResponse::ApiError(errors) => {
                    println!("Response Errors: \n {:#?}", &errors);
                }
            }
        }
        Err(error) => {
            println!("error \n {:#?}", error);
        }
    }
}
