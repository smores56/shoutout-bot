use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};
use shoutout::Shoutout;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

mod shoutout;

struct ShoutoutBot {
    pool: PgPool,
    guild_id: String,
}

#[async_trait]
impl EventHandler for ShoutoutBot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::welcome::register(command))
                .create_application_command(|command| commands::numberinput::register(command))
                .create_application_command(|command| commands::attachmentinput::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::wonderful_command::register(command)
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = get_secret("DISCORD_TOKEN", &secret_store)?;
    let guild_id = get_secret("GUILD_ID", &secret_store)?;

    Shoutout::create_table_if_missing(&pool).await?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(ShoutoutBot { pool, guild_id })
        .await
        .context("Failed to create Discord client")?;

    Ok(client.into())
}

fn get_secret(key: &str, store: &SecretStore) -> anyhow::Result<String> {
    store
        .get(key)
        .with_context(|| format!("`{key}` was not found in secrets"))
}
