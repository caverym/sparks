use discord::model::{Event, Game, Message, OnlineStatus, User, UserId};
use sparks::{error::Error, token::Token, Bot};

fn main() -> Result<(), Error> {
    fn get_token() -> Result<Token, Error> {
        use std::fs::File;
        use std::io::Read;
        let mut file: File = File::open("token")?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        let string: String = String::from_utf8(buf)?;
        Ok(Token::new("bot", string))
    }
    let mut sparks: Bot = Bot::connect(get_token()?)?;

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
        _ => (),
    }
    Ok(())
}

fn message(sparks: &mut Bot, message: Message) -> Result<(), Error> {
    if message.author.bot {
        return Ok(());
    }

    let content: Vec<&str> = message.content.split(' ').collect();
    match content[0] {
        "s!help" => sparks.command(help, &message)?,
        "s!sysinfo" => sparks.command(sysinfo, &message)?,
        "s!avatar" => sparks.command(avatar, &message)?,
        "s!flip" => sparks.command(flip, &message)?,
        "s!blow" => sparks.command(blow, &message)?,
        _ => {}
    }

    Ok(())
}

#[cfg(unix)]
fn sysinfo(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    use uname_rs::*;
    let uts: Uname = Uname::new()?;

    sparks.handle.send_message(
        message.channel_id,
        &format!(
            "{} {} {} {} {}",
            uts.sysname, uts.nodename, uts.release, uts.version, uts.machine
        ),
        "",
        false,
    )?;
    Ok(())
}

#[cfg(windows)]
fn sysinfo(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    use uname_rs::*;
    let uts: Uname = Uname::new()?;

    sparks
        .handle
        .send_message(message.channel_id, "binbows", "", false)?;
    Ok(())
}

fn help(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    sparks
        .handle
        .send_message(message.channel_id, "You get no help", "", false)?;
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
        sparks
            .handle
            .send_message(message.channel_id, &url, "", false)?;
    }

    Ok(())
}

fn flip(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    let num: u8 = rand::random::<u8>() % 2;
    let coin: &str = if num == 0 { "heads" } else { "tails" };
    sparks
        .handle
        .send_message(message.channel_id, coin, "", false)?;
    Ok(())
}
