use sparks::{
    Bot,
    error::Error,
    token::Token,
};
use discord::model::{Event, Message, UserId};

fn get_token() -> Result<Token, Error> {
    use std::io::Read;
    use std::fs::File;
    let mut file: File = File::open("token")?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    let string: String = String::from_utf8(buf)?;
    Ok(Token::new("bot", string))
}

fn main() -> Result<(), Error> {
    let mut sparks: Bot = Bot::connect(get_token()?)?;
    sparks.run(matcher)?;
    sparks.disconnect();
    Ok(())
}

fn matcher(sparks: &mut Bot, event: Event) -> Result<(), Error> {
    match event {
        Event::Ready(r) => {
            println!("Ready!\n{:#?}", r);
            sparks.ready_event = r;
        },
        Event::MessageCreate(m) => message(sparks, m)?,
        _ => (),
    }
    Ok(())
}

fn message(sparks: &mut Bot, message: Message) -> Result<(), Error> {
    println!("message by {:#?}", message.author);
    println!("{}", message.content);

    if message.author.bot {
        return Ok(());
    }

    if message.content.contains("🐝") {
        sparks.command(bee, &message)?;
    }

    let content: Vec<&str> = message.content.split(' ').collect();
    match content[0] {
        "s!help" => sparks.command(help, &message)?,
        "s!sysinfo" => sparks.command(sysinfo, &message)?,
        _ => {}
    }

    Ok(())
}

fn sysinfo(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    use uname_rs::*;
    let uts: Uname = Uname::new()?;
    sparks.handle.send_message(message.channel_id, &uts.release, "", false)?;
    Ok(())
}

fn help(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    sparks.handle.send_message(message.channel_id, "You get no help", "", false)?;
    Ok(())
}

fn bee(sparks: &mut Bot, message: &Message) -> Result<(), Error> {
    sparks.handle.send_message(message.channel_id, "🐝", "", false)?;
    Ok(())
}