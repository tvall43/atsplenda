use serenity::model::user::User;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use serenity::model::id::ChannelId;
use serenity::utils::MessageBuilder;
use serenity::builder::{CreateAttachment, CreateMessage};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		// Don't respond to messages from other bots.
		if msg.author.bot {
			return;
		}
		// Trigger-based command responses.
		if msg.content == "bitch" {
			let _ = msg.channel_id.say(&ctx.http, "LASAGNA!!!").await;
		}
		if msg.content == "uwu" {
			let _ = msg.channel_id.say(&ctx.http, "OwO").await;
		}
		if msg.content == "oof" {
			let _ = msg.channel_id.say(&ctx.http, "no").await;
		}
		if msg.content == "mallcop" {
			let _ = msg.channel_id.say(&ctx.http, "A mustache a day keeps the shoplifters at bay").await;
		}
		if msg.content == "nooo" {
			let nogif = CreateAttachment::path("resources/no.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, nogif, builder).await;
		}
		if msg.content == "bad bot" {
			let _ = msg.channel_id.say(&ctx.http, "you're not my dad").await;
		}
		if msg.content == "@spnexa" {
			let _ = msg.channel_id.say(&ctx.http, "someone spnexad").await;
		}
		if msg.content == "have a cookie" {
			let _ = msg.channel_id.say(&ctx.http, "I LOVE COOKIES").await;
		}
		if msg.content == ":)))" {
			let _ = msg.channel_id.say(&ctx.http, "Woah, calm yo tits there. no one has that many chins").await;
		}
		if msg.content == "f" {
			let fgif = CreateAttachment::path("resources/F.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, fgif, builder).await;
		}
		if msg.content == "wow" {
			let wowgif = CreateAttachment::path("resources/wow.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, wowgif, builder).await;
			let mindblowngif = CreateAttachment::path("resources/mindblown.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, mindblowngif, builder).await;
			let surprisedgif = CreateAttachment::path("resources/surprised.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, surprisedgif, builder).await;
		}
		if msg.content == "peter" {
			let eightbitpetermp4 = CreateAttachment::path("resources/8bitpeter.mp4").await;
			let builder = CreateMessage::new().content("bestgif");
			let _ = msg.channel_id.send_files(&ctx.http, eightbitpetermp4, builder).await;
		}
		if msg.content == "peter.mp4" {
			let eightbitpetermp4 = CreateAttachment::path("resources/8bitpeter.mp4").await;
			let builder = CreateMessage::new().content("bestgif");
			let _ = msg.channel_id.send_files(&ctx.http, eightbitpetermp4, builder).await;
		}
		if msg.content == "bestgif" {
			let eightbitpetermp4 = CreateAttachment::path("resources/8bitpeter.mp4").await;
			let builder = CreateMessage::new().content("bestgif");
			let _ = msg.channel_id.send_files(&ctx.http, eightbitpetermp4, builder).await;
		}
		if msg.content == "fedora" {
			let _ = msg.channel_id.say(&ctx.http, "let's play a game").await;
			let _ = msg.channel_id.say(&ctx.http, "https://tvall.asshatgaming.com/gameboy/splendashairsalon.gb").await;
		}
		if msg.content == "😦" {
			let theretheregif = CreateAttachment::path("resources/therethere.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, theretheregif, builder).await;
		}
		if msg.content == "🙂" {
			let _ = msg.channel_id.say(&ctx.http, "woah there, too much happiness up in here. calm yo tits.").await;
		}
		if msg.content == "😐" {
			let _ = msg.channel_id.say(&ctx.http, "cammy wanted a response to this one, so here ya go").await;
		}
		if msg.content == "botcommandthing" {
			let _ = msg.channel_id.say(&ctx.http, ";)").await;
		}
		if msg.content == "!deleteme" {
			let _ = msg.channel_id.say(&ctx.http, "I will delete myself now...").await;
		}
		if msg.content == "its quiet" {
			let antisocialgif = CreateAttachment::path("resources/antisocial.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, antisocialgif, builder).await;
		}
		if msg.content == "it\'s quiet" {
			let antisocialgif = CreateAttachment::path("resources/antisocial.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, antisocialgif, builder).await;
		}
		if msg.content == "polygraph" {
			let _ = msg.channel_id.say(&ctx.http, "lie detectors are a lie").await;
		}
		if msg.content == "getoptifine" {
			let _ = msg.channel_id.say(&ctx.http, "here ya go").await;
			let _ = msg.channel_id.say(&ctx.http, "https://optifine.net/downloads").await;
		}
		if msg.content == "advertise" {
			let boobsjpg = CreateAttachment::path("resources/boobs.jpg").await;
			let builder = CreateMessage::new().content("THIS IS AN ADVERTISEMENT");
			let _ = msg.channel_id.send_files(&ctx.http, boobsjpg, builder).await;
		}
		if msg.content == "mobileshrug" {
			let shruggif = CreateAttachment::path("resources/shrug.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, shruggif, builder).await;
		}
		if msg.content == "deletedtvallresponse" {
			let deletegif = CreateAttachment::path("resources/delete.gif").await;
			let builder = CreateMessage::new().content("you deleted him");
			let _ = msg.channel_id.send_files(&ctx.http, deletegif, builder).await;
		}
		if msg.content == "nope" {
			let nopegif = CreateAttachment::path("resources/nope.gif").await;
			let builder = CreateMessage::new();
			let _ = msg.channel_id.send_files(&ctx.http, nopegif, builder).await;
		}
		// Temporary "todo" command for public tracking of missing original Python bot functionality.
		if msg.content == "todo" {
			let _ = msg.channel_id.say(&ctx.http, "1. Add the cancer counter code.").await;
			let _ = msg.channel_id.say(&ctx.http, "2. Create the @splenda command code to mention SugarD-x.").await;
			let _ = msg.channel_id.say(&ctx.http, "3. Create the code to mention SugarD-x when AtSplenda is mentioned.").await;
			let _ = msg.channel_id.say(&ctx.http, "4. Create the anonymous command code to mention SugarD-x.").await;
			let _ = msg.channel_id.say(&ctx.http, "5. Add the code for searching xkcd").await;
			let _ = msg.channel_id.say(&ctx.http, "6. Add the code for searching Cyanide & Happiness").await;
			let _ = msg.channel_id.say(&ctx.http, "7. Add the oof command cancer code.").await;
		}
		let author = msg.author;
		let content = msg.content;
		if content.starts_with("I'm ") {
			let _ = msg.channel_id.say(&ctx.http, format!("Hi {author}, I'm dad.")).await;
		}
		if content.starts_with("XD") {
			let _ = msg.channel_id.say(&ctx.http, "lulz").await;
		}
		if content.starts_with("D:") {
			let _ = msg.channel_id.say(&ctx.http, "shocking").await;
		}
	}

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
		let channel_id = ChannelId::from(510553881764298766 as u64);
		let welcome_message = MessageBuilder::new()
			.push("hey look, it's a ")
			.mention(&new_member)
			.build();
		let _ = channel_id.say(&ctx.http,&welcome_message).await;
    }

    async fn guild_member_removal(&self, ctx: Context, _guild_id: GuildId, user: User, _member_data_if_available: Option<Member>) {
		let channel_id = ChannelId::from(510553881764298766 as u64);

		let username = if let Some(global_name) = &user.global_name {
			global_name.clone()
		} else {
			user.name.clone()
		};
		let leave_message = MessageBuilder::new()
			.push("goodbye ")
			.push(&username)
			.build();
		let _ = channel_id.say(&ctx.http,&leave_message).await;
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Id's, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        use serenity::gateway::ActivityData;
        use serenity::model::user::OnlineStatus;

	// Set the presence activity data and online status.
        let game = ActivityData::playing("Minecraft, but for bots");
        let status = OnlineStatus::Idle;

        ctx.set_presence(Some(game), status);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = "DISCORD_TOKEN";
    // Set gateway intents, which decides what events the bot will be notified about.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}