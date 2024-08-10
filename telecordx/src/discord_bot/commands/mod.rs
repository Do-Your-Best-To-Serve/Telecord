pub mod ping;

use serenity::builder::CreateAutocompleteResponse;

fn get_all_command_registers() -> Vec<fn() -> CreateAutocompleteResponse> {
  vec![
    ping::register
  ]
}