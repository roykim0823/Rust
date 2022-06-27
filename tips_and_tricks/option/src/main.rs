fn main() {
    option();   println!();
    act_on_option();    println!();
}

#[derive(Debug, Clone)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invitation {
        Invitation {
            invitee,
            attending,
            message,
        }
    }
}

fn option() {
    let invitation_1 = Invitation::new("Dolores".to_string(), true, None);
    let invitation_2 = Invitation::new("Cosmo".to_string(), false, Some("Sorry. I going to be flying to Mars that week".to_string()));

    println!("{:#?}", invitation_1);
    println!("{:#?}", invitation_2);
    println!();
    println!("{:?}", invitation_2);
}

fn filter_1(invitations: Vec<Invitation>) -> Vec<String> {
    let mut result = vec![];

    for invite in invitations {
        if invite.attending == false && invite.message.is_none() {
            result.push(invite.invitee.clone());
        }
    }
    result
}

fn filter_2(invitations: Vec<Invitation>) -> Vec<String> {
    let mut result = vec![];
    
    for invite in invitations {
        if invite.attending == true && invite.message.is_some() {
            result.push(invite.message.unwrap().clone());
        }
    }
    result
}

fn act_on_option() {
    let list_of_responses = vec![
        Invitation::new(
            "Rex".to_string(),
            true,
            Some("Can't wait to see everyone!".to_string()),
        ),
        Invitation::new(
            "Dolores".to_string(),
            true,
            Some("I am ready to PARTY!!!".to_string()),
        ),
        Invitation::new("Bubbles".to_string(), true, None),
        Invitation::new("Eileen".to_string(), true, None),
        Invitation::new("Belle".to_string(), false, None),
        Invitation::new(
            "Cosmo".to_string(),
            false,
            Some("Can't make it. I'll be on Mars".to_string()),
        ),
        Invitation::new(
            "Dolly".to_string(),
            false,
            Some("I have a marathon run that day".to_string()),
        ),
        Invitation::new("Elton".to_string(), false, None),
    ];

    dbg!(filter_1(list_of_responses.clone()));
    dbg!(filter_2(list_of_responses.clone()));
}