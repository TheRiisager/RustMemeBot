use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::db::*;


const HELP_MESSAGE: &str = "
Hello!

this is but a simple bot

!give LINK - adds a meme to the bot, please provide a valid, direct image url

!post - posts a random meme
";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let split_string: Vec<&str> = msg.content.split(' ').collect();
        let cmd = split_string[0];

        match cmd {
            "!help" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!post" => {
                let mut db_handler = Database::new();
                let meme = db_handler.get_meme();
                match meme {
                    Ok(meme) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, meme.link).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    Err(error) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, error).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
                drop(db_handler);
            }
            "!give" => {
                let mut db_handler = Database::new();
                let request = db_handler.add_meme(split_string[1]);
                match request {
                    Ok(_) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Thank you!").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    Err(error) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, error).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
                drop(db_handler);
            }
            _ => println!("not a command, skipping"),
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}