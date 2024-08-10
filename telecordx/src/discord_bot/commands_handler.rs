use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::prelude::*;
use std::collections::HashMap;

pub async fn handle_interaction(ctx: &Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        println!("收到指令交互: {command:#?}");

        let command_handlers: HashMap<&str, fn(&[serenity::model::application::command::CommandDataOption]) -> String> = [
            ("ping", commands::ping::run),
            ("id", commands::id::run),
            ("attachmentinput", commands::attachmentinput::run),
        ]
        .iter()
        .cloned()
        .collect();

        let result = if let Some(handler) = command_handlers.get(command.data.name.as_str()) {
            Ok(handler(&command.data.options()))
        } else if command.data.name == "modal" {
            match commands::modal::run(ctx, &command).await {
                Ok(_) => Ok(None),
                Err(e) => Err(format!("無法處理 modal 命令: {e}")),
            }
        } else {
            Ok(Some("尚未實現 :(".to_string()))
        };

        match result {
            Ok(Some(content)) => {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    eprintln!("無法回應斜線命令: {why}");
                }
            }
            Ok(None) => {} // Do nothing if there's no content to send
            Err(error_message) => {
                let data = CreateInteractionResponseMessage::new().content(error_message);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    eprintln!("無法回應斜線命令: {why}");
                }
            }
        }
    }
}
