pub mod error;
pub mod token;

use error::Error;
use token::Token;

use discord::model::{ChannelId, Event, Game, Message, OnlineStatus, ReadyEvent};
use discord::{Connection, Discord};
use std::fmt::Arguments;

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
        f: F,
    ) -> Result<Event, Error> {
        loop {
            let env: Event = self.connection.recv_event()?;
            f(self, env)?;
        }
    }

    pub fn set_presence(&mut self, game: Option<Game>, status: OnlineStatus, afk: bool) {
        self.connection.set_presence(game, status, afk)
    }

    pub fn send_message<T: ToString>(
        &mut self,
        channel_id: ChannelId,
        message: T,
    ) -> Result<(), Error> {
        self.handle
            .send_message(channel_id, message.to_string().as_str(), "", false)?;
        Ok(())
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
