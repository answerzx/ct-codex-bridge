use clap::{Parser, Subcommand};
use ct_codex_bridge::{auth, codex, config, server};
use std::path::PathBuf;

const PORT: u16 = 8787;

#[derive(Debug, Parser)]
#[command(name = "ct-codex-bridge")]
#[command(about = "LAN Web control panel for switching CT-managed Codex accounts")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the LAN Web server.
    Serve,
    /// Initialize or update the Web panel password.
    SetupPassword {
        /// Pass the password non-interactively. Prefer the prompt for normal use.
        #[arg(long)]
        password: Option<String>,
    },
    /// Install a user LaunchAgent that starts the bridge at login.
    InstallLaunchAgent,
    /// Unload and remove the user LaunchAgent.
    UninstallLaunchAgent,
    /// Print resolved paths and current service status.
    Doctor,
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Command::Serve) {
        Command::Serve => server::serve(PORT).await,
        Command::SetupPassword { password } => setup_password(password),
        Command::InstallLaunchAgent => install_launch_agent(),
        Command::UninstallLaunchAgent => uninstall_launch_agent(),
        Command::Doctor => doctor(),
    }
}

fn setup_password(password: Option<String>) -> Result<(), String> {
    let password = match password {
        Some(value) => value,
        None => {
            let first = rpassword::prompt_password("Panel password: ")
                .map_err(|error| format!("failed to read password: {error}"))?;
            let second = rpassword::prompt_password("Confirm password: ")
                .map_err(|error| format!("failed to read password confirmation: {error}"))?;
            if first != second {
                return Err("passwords do not match".to_string());
            }
            first
        }
    };

    if password.trim().len() < 6 {
        return Err("password must be at least 6 characters".to_string());
    }

    let mut cfg = config::BridgeConfig::load_or_default()?;
    cfg.password_hash = Some(auth::hash_password(&password)?);
    cfg.save()?;
    println!("Password saved to {}", config::config_path()?.display());
    Ok(())
}

fn install_launch_agent() -> Result<(), String> {
    let source_exe =
        std::env::current_exe().map_err(|error| format!("resolve current exe: {error}"))?;
    let exe = install_service_binary(&source_exe)?;
    let plist_path = launch_agent_path()?;
    let log_dir = config::config_dir()?.join("logs");
    std::fs::create_dir_all(&log_dir).map_err(|error| format!("create log dir: {error}"))?;
    let stdout_log = log_dir.join("launchd.out.log");
    let stderr_log = log_dir.join("launchd.err.log");

    let content = launch_agent_plist(&exe, &stdout_log, &stderr_log);
    if let Some(parent) = plist_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("create launch agent dir: {error}"))?;
    }
    codex::write_string_atomic(&plist_path, &content)?;

    let _ = std::process::Command::new("launchctl")
        .args(["unload", plist_path.to_string_lossy().as_ref()])
        .output();
    let output = std::process::Command::new("launchctl")
        .args(["load", "-w", plist_path.to_string_lossy().as_ref()])
        .output()
        .map_err(|error| format!("run launchctl load: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "launchctl load failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    println!("Installed service binary: {}", exe.display());
    println!("Installed LaunchAgent: {}", plist_path.display());
    println!("Panel URL: http://0.0.0.0:{PORT}");
    Ok(())
}

fn uninstall_launch_agent() -> Result<(), String> {
    let plist_path = launch_agent_path()?;
    if plist_path.exists() {
        let _ = std::process::Command::new("launchctl")
            .args(["unload", plist_path.to_string_lossy().as_ref()])
            .output();
        std::fs::remove_file(&plist_path)
            .map_err(|error| format!("remove {}: {error}", plist_path.display()))?;
        println!("Removed LaunchAgent: {}", plist_path.display());
    } else {
        println!("LaunchAgent was not installed: {}", plist_path.display());
    }
    Ok(())
}

fn doctor() -> Result<(), String> {
    let cfg = config::BridgeConfig::load_or_default()?;
    let paths = codex::ResolvedPaths::new()?;
    println!("config: {}", config::config_path()?.display());
    println!("password configured: {}", cfg.password_hash.is_some());
    println!("ct index: {}", paths.account_index_path.display());
    println!("ct accounts dir: {}", paths.accounts_dir.display());
    println!("codex home: {}", paths.codex_home.display());
    println!("codex app: {}", paths.codex_app_path.display());
    println!("service binary: {}", service_binary_path()?.display());
    println!("launch agent: {}", launch_agent_path()?.display());
    Ok(())
}

fn install_service_binary(source_exe: &std::path::Path) -> Result<PathBuf, String> {
    let destination = service_binary_path()?;
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("create service binary dir: {error}"))?;
    }

    if source_exe != destination {
        std::fs::copy(source_exe, &destination).map_err(|error| {
            format!(
                "copy service binary {} -> {}: {error}",
                source_exe.display(),
                destination.display()
            )
        })?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&destination)
            .map_err(|error| format!("read service binary metadata: {error}"))?
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&destination, permissions)
            .map_err(|error| format!("set service binary permissions: {error}"))?;
    }

    Ok(destination)
}

fn service_binary_path() -> Result<PathBuf, String> {
    Ok(config::config_dir()?.join("bin").join("ct-codex-bridge"))
}

fn launch_agent_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "unable to resolve home directory".to_string())?;
    Ok(home
        .join("Library")
        .join("LaunchAgents")
        .join("com.ct-codex-bridge.plist"))
}

fn launch_agent_plist(
    exe: &std::path::Path,
    stdout_log: &std::path::Path,
    stderr_log: &std::path::Path,
) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.ct-codex-bridge</string>
  <key>ProgramArguments</key>
  <array>
    <string>{}</string>
    <string>serve</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
  <key>StandardOutPath</key>
  <string>{}</string>
  <key>StandardErrorPath</key>
  <string>{}</string>
</dict>
</plist>
"#,
        escape_xml(&exe.to_string_lossy()),
        escape_xml(&stdout_log.to_string_lossy()),
        escape_xml(&stderr_log.to_string_lossy())
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
