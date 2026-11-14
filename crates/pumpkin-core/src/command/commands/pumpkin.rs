use pumpkin_data::packet::CURRENT_MC_VERSION;
use pumpkin_util::text::{TextComponent, color::NamedColor};
use pumpkin_util::PermissionLvl;
use pumpkin_util::permission::{Permission, PermissionDefault, PermissionRegistry};

use crate::command::argument_builder::{command, ArgumentBuilder};
use crate::command::context::command_context::CommandContext;
use crate::command::node::dispatcher::CommandDispatcher;
use crate::command::node::{CommandExecutor, CommandExecutorResult};

const DESCRIPTION: &str = "Display information about Melon MC.";
const PERMISSION: &str = "melon:command.melon";
const PERMISSION_PUMPKIN: &str = "pumpkin:command.pumpkin";

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

struct Executor;

impl CommandExecutor for Executor {
    fn execute(&self, context: &CommandContext) -> CommandExecutorResult {
        let mut msg = TextComponent::text("Melon MC ")
            .color_named(NamedColor::Gold)
            .bold();

        msg = msg.add_child(
            TextComponent::text(format!("v{} ({})\n", CARGO_PKG_VERSION, GIT_HASH))
                .color_named(NamedColor::Green),
        );

        msg = msg.add_child(
            TextComponent::text(format!(
                "Minecraft: {} (Protocol {})",
                CURRENT_MC_VERSION,
                CURRENT_MC_VERSION.protocol_version()
            ))
            .color_named(NamedColor::Aqua),
        );

        context.source.send_message(msg);
        Ok(1)
    }
}

pub fn register(dispatcher: &mut CommandDispatcher, registry: &PermissionRegistry) {
    registry.register_permission_or_panic(Permission::new(
        PERMISSION,
        DESCRIPTION,
        PermissionDefault::Op(PermissionLvl::Zero),
    ));

    registry.register_permission_or_panic(Permission::new(
        PERMISSION_PUMPKIN,
        DESCRIPTION,
        PermissionDefault::Op(PermissionLvl::Zero),
    ));

    dispatcher.register_with_aliases(
        command("melon", DESCRIPTION)
            .requires(PERMISSION)
            .executes(Executor),
        &["pumpkin", "version"],
    );
}
