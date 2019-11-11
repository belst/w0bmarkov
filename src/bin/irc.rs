use irc::client::prelude::*;
use failure::Error;
use w0bmarkov::*;
use w0bmarkov::diesel::sql_query;
use w0bmarkov::diesel::QueryableByName;

#[derive(Debug,PartialEq,PartialOrd,Clone, QueryableByName)]
struct Msg {
    #[sql_type = "diesel::sql_types::Text"]
    txt: String
}

fn main() -> Result<(), Error> {
    let dbconn = establish_connection();
    let config = Config::load("ircconfig.toml")?;

    let mut reactor = IrcReactor::new()?;
    let client = reactor.prepare_client_and_connect(&config)?;
    client.identify()?;

    reactor.register_client_with_handler(client, move |client, message| {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.starts_with(".sagwas") {
                let reply: Vec<Msg> = sql_query(include_str!("../getsentence.sql"))
                    .load(&dbconn).unwrap();
                if reply.len() > 0 {
                    client.send_privmsg(target, &reply[0].txt)?;
                }
            }
        }
        Ok(())
    });

    reactor.run()?;
    
    Ok(())
}