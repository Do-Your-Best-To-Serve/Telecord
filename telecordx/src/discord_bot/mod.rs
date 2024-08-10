mod commands;
mod commands_handler;

use std::env;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        commands_handler::handle_interaction(&ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} 已連接!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("需要在環境中設定 GUILD_ID")
                .parse()
                .expect("GUILD_ID 必須是整數"),
        );

        let command_registers = commands::get_all_command_registers();

        let mut registered_commands = vec![];
        
        for register in command_registers {
            registered_commands.push(register());
        }

        let commands = guild_id
            .set_commands(&ctx.http, registered_commands)
            .await;

        println!("現在我有以下公會斜線命令: {commands:#?}");
        
        // let guild_command =
        //     Command::create_global_command(&ctx.http, commands::wonderful_command::register())
        //         .await;

        // println!("我創建了以下全局斜線命令: {guild_command:#?}");
    }
}
