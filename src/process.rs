use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

pub fn codex_pids() -> Result<Vec<u32>, String> {
    let output = std::process::Command::new("pgrep")
        .args(["-f", "Codex.app/Contents/MacOS/Codex"])
        .output()
        .map_err(|error| format!("run pgrep: {error}"))?;

    if !output.status.success() && output.stdout.is_empty() {
        return Ok(Vec::new());
    }

    let current_pid = std::process::id();
    let mut pids = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .filter(|pid| *pid != 0 && *pid != current_pid)
        .collect::<Vec<_>>();
    pids.sort_unstable();
    pids.dedup();
    Ok(pids)
}

pub fn restart_codex(configured_app_path: &Path) -> Result<bool, String> {
    let pids = codex_pids()?;
    for pid in &pids {
        close_pid(*pid, Duration::from_secs(20))?;
    }
    start_codex(configured_app_path)?;
    Ok(true)
}

fn close_pid(pid: u32, timeout: Duration) -> Result<(), String> {
    if !is_pid_running(pid) {
        return Ok(());
    }

    let output = std::process::Command::new("kill")
        .args(["-15", &pid.to_string()])
        .output()
        .map_err(|error| format!("run kill -15 {pid}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "kill -15 {pid} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let started = Instant::now();
    while started.elapsed() < timeout {
        if !is_pid_running(pid) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(350));
    }
    Err(format!(
        "Codex process {pid} did not exit within {}s",
        timeout.as_secs()
    ))
}

fn start_codex(configured_app_path: &Path) -> Result<(), String> {
    let app_root = normalize_app_root(configured_app_path)
        .or_else(|| Some(PathBuf::from("/Applications/Codex.app")))
        .ok_or_else(|| "unable to resolve Codex.app path".to_string())?;
    if !app_root.exists() {
        return Err(format!("Codex.app not found at {}", app_root.display()));
    }

    let output = std::process::Command::new("open")
        .arg("-n")
        .arg("-a")
        .arg(&app_root)
        .output()
        .map_err(|error| format!("run open -n -a {}: {error}", app_root.display()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "open Codex failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn is_pid_running(pid: u32) -> bool {
    std::process::Command::new("kill")
        .args(["-0", &pid.to_string()])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn normalize_app_root(path: &Path) -> Option<PathBuf> {
    let raw = path.to_string_lossy();
    raw.find(".app")
        .map(|index| PathBuf::from(&raw[..index + 4]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_bundle_root_from_executable_path() {
        assert_eq!(
            normalize_app_root(Path::new("/Applications/Codex.app/Contents/MacOS/Codex")).unwrap(),
            PathBuf::from("/Applications/Codex.app")
        );
        assert_eq!(
            normalize_app_root(Path::new("/Applications/Codex.app")).unwrap(),
            PathBuf::from("/Applications/Codex.app")
        );
    }
}
