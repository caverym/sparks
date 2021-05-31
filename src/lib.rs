pub mod error;
pub mod token;

use error::Error;
use token::Token;

use discord::model::{Event, Message, ReadyEvent};
use discord::{Connection, Discord};

pub struct Bot {
    token: String,
    pub handle: Discord,
    pub connection: Connection,
    pub ready_event: ReadyEvent,
}

impl Bot {
    pub fn connect(token: Token) -> Result<Bot, Error> {
        let token: String = token.to_string();
        let handle: Discord = Discord::from_bot_token(&token)?;
        let (connection, ready_event) = handle.connect()?;

        Ok(Bot {
            token,
            handle,
            connection,
            ready_event,
        })
    }

    pub fn run<F: Fn(&mut Bot, Event) -> Result<(), Error>>(
        &mut self,
        matcher: F,
    ) -> Result<Event, Error> {
        loop {
            let env: Event = self.connection.recv_event()?;
            matcher(self, env)?;
        }
    }

    pub fn command<F: Fn(&mut Bot, &Message) -> Result<(), Error>>(
        &mut self,
        f: F,
        message: &Message,
    ) -> Result<(), Error> {
        f(self, message)
    }

    pub fn disconnect(self) {
        drop(self)
    }
}

#[cfg(test)]
mod tests {}
