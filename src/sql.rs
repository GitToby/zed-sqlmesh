use zed_extension_api::{self as zed, LanguageServerId, Result};

struct SQLExtension;

impl zed::Extension for SQLExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("sqlmesh_lsp")
            .ok_or_else(|| "sqlmesh_lsp not found in PATH. Install sqlmesh with: pip install sqlmesh".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(SQLExtension);
