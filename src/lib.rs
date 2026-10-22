use zed_extension_api::{self as zed, Result};

struct GooseExtension;

impl zed::Extension for GooseExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("goose")
            .ok_or_else(|| "goose is not installed".to_string())?;
        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GooseExtension);
