use discord::model::{
    ChannelId, Event, Game, Member, Message, OnlineStatus, PossibleServer, ServerId, User,
};
use sparks::{error::Error, token::Token, Bot};
use std::io::{Read, Seek};

fn main() {
    use std::process::exit;
    if let Err(e) = sparks() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn sparks() -> Result<(), Error> {
    fn get(p: &str) -> Result<String, Error> {
        use std::fs::File;
        use std::io::Read;
        let mut file: File = File::open(p)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        let string: String = String::from_utf8(buf)?;
        Ok(string)
    }

    let token: String = get("token")?;
    let owner: String = get("owner")?;

    let mut sparks: Bot = Bot::connect(Token::new("bot", token))?;

    sparks.register_owner(owner);

    sparks.set_presence(
        Some(Game::playing("rustc".to_string())),
        OnlineStatus::Online,
        false,
    );

    sparks.run(matcher)?;

    sparks.disconnect();

    Ok(())
}

fn matcher(sparks: &mut Bot, event: Event) -> Result<(), Error> {
    match event {
        Event::MessageCreate(m) => message(sparks, m)?,
        Event::Unknown(string, object) => println!("unknown! {}\n{:#?}", string, object),
        _ => (),
    }
    Ok(())
}

fn message(sparks: &mut Bot, message: Message) -> Result<(), Error> {
    if message.author.bot {
        return Ok(());
    }

    println!(
        "message sent by {} in {} at {}",
        message.author.name, message.channel_id, message.timestamp
    );

    let content: Vec<&str> = message.content.split(' ').collect();
    match content[0] {
        "s!help" => sparks.command(help, &message)?,
        "s!avatar" => sparks.command(avatar, &message)?,
        "s!flip" => sparks.command(flip, &message)?,
        "s!info" => sparks.command(info, &message)?,
        _ => {}
    }

    Ok(())
}

fn info(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    if let Some(owner) = sparks.owner {
        sparks.send_message(message.channel_id, format!("ask <@{}>", owner))?;
    } else {
        sparks.send_message(message.channel_id, "I really have nothing to say")?;
    }
    Ok(())
}

fn help(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    sparks.send_message(message.channel_id, "You get no help")?;
    Ok(())
}

fn avatar(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    let mut users: Vec<User> = Vec::new();

    if message.mentions.is_empty() {
        users.append(&mut vec![message.author.clone()])
    } else {
        let mut tmp = message.mentions.to_owned();
        users.append(&mut tmp);
    }

    for user in users {
        let url: String = user
            .avatar_url()
            .unwrap_or(format!("can't get {}'s avatar", user.name));
        sparks.send_message(message.channel_id, url)?;
    }

    Ok(())
}

fn flip(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    let num: u8 = rand::random::<u8>() % 2;
    let coin: &str = if num == 0 { "heads" } else { "tails" };
    sparks.send_message(message.channel_id, coin)?;
    Ok(())
}
