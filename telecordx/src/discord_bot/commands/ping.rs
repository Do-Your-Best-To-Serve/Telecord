use serenity::builder::CreateApplicationCommand;

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("ping")
        .description("A ping command")
}

pub fn run(options: &[serenity::model::application::command::CommandDataOption]) -> String {
    // Your ping command execution logic here
    "Pong!".to_string()
}
