use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::process::ExitCode;

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

// ── Harpoon file helpers ──────────────────────────────────────────────────────

fn harpoon_file_path() -> Result<PathBuf> {
  let root = env::var("ZED_WORKTREE_ROOT").context("ZED_WORKTREE_ROOT is not set")?;
  let mut h = Sha256::new();
  h.update(root.as_bytes());
  let name = format!("{:x}.harpoon", h.finalize());
  Ok(zed_config_dir()?.join("harpoon").join(name))
}

#[cfg(target_os = "windows")]
fn zed_config_dir() -> Result<PathBuf> {
  env::var_os("APPDATA")
    .map(PathBuf::from)
    .map(|path| path.join("Zed"))
    .context("APPDATA is not set")
}

#[cfg(target_os = "linux")]
fn zed_config_dir() -> Result<PathBuf> {
  if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
    return Ok(PathBuf::from(config_home).join("zed"));
  }

  env::var_os("HOME")
    .map(PathBuf::from)
    .map(|path| path.join(".config/zed"))
    .context("HOME is not set")
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn zed_config_dir() -> Result<PathBuf> {
  env::var_os("HOME")
    .map(PathBuf::from)
    .map(|path| path.join(".config/zed"))
    .context("HOME is not set")
}

fn read_entries(path: &Path) -> Result<Vec<String>> {
  match fs::File::open(path) {
    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(vec![]),
    Err(e) => Err(anyhow::Error::from(e)),
    Ok(f) => {
      let mut entries = Vec::new();
      for line in io::BufReader::new(f).lines() {
        let l = line?.trim().to_string();
        if !l.is_empty() {
          validate_harpoon_entry(&l)?;
          entries.push(l);
        }
      }
      Ok(entries)
    }
  }
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
  if let Some(dir) = path.parent() {
    fs::create_dir_all(dir).context("cannot create harpoon dir")?;
  }
  Ok(())
}

fn validate_harpoon_entry(entry: &str) -> Result<()> {
  if entry.is_empty() {
    bail!("harpoon entry is empty");
  }

  if entry.contains(['\n', '\r']) {
    bail!("harpoon entry contains invalid newline characters");
  }

  let rel = Path::new(entry);
  if has_windows_drive_prefix(entry)
    || entry.starts_with('\\')
    || rel.is_absolute()
    || rel.components().any(|component| {
      matches!(
        component,
        Component::ParentDir | Component::Prefix(_) | Component::RootDir
      )
    })
  {
    bail!("unsafe path in harpoon entry: {entry}");
  }

  Ok(())
}

fn has_windows_drive_prefix(entry: &str) -> bool {
  let bytes = entry.as_bytes();
  bytes.len() >= 3
    && bytes[0].is_ascii_alphabetic()
    && bytes[1] == b':'
    && matches!(bytes[2], b'\\' | b'/')
}

// ── Zed CLI ───────────────────────────────────────────────────────────────────

fn open_in_zed(path: &Path) -> Result<i32> {
  let status = Command::new(zed_cli_path())
    .arg(path)
    .status()
    .context("failed to run Zed CLI")?;

  Ok(status.code().unwrap_or(1))
}

fn zed_cli_path() -> PathBuf {
  env::var_os("ZED_CLI_PATH")
    .map(PathBuf::from)
    .or_else(zed_app_cli_path)
    .unwrap_or_else(default_zed_cli_path)
}

#[cfg(target_os = "macos")]
fn zed_app_cli_path() -> Option<PathBuf> {
  env::var_os("ZED_APP_PATH").map(|path| PathBuf::from(path).join("Contents/MacOS/cli"))
}

#[cfg(not(target_os = "macos"))]
const fn zed_app_cli_path() -> Option<PathBuf> {
  None
}

#[cfg(target_os = "macos")]
fn default_zed_cli_path() -> PathBuf {
  PathBuf::from("/Applications/Zed.app/Contents/MacOS/cli")
}

#[cfg(target_os = "windows")]
fn default_zed_cli_path() -> PathBuf {
  PathBuf::from("zed.exe")
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn default_zed_cli_path() -> PathBuf {
  PathBuf::from("zed")
}

// ── Commands ──────────────────────────────────────────────────────────────────

fn harpoon_open() -> Result<i32> {
  let path = harpoon_file_path()?;
  ensure_parent_dir(&path)?;
  // Touch the file so Zed can open it even if no entries exist yet.
  fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&path)
    .context("cannot create harpoon file")?;
  open_in_zed(&path)
}

fn harpoon_add() -> Result<i32> {
  let rel = env::var("ZED_RELATIVE_FILE").context("ZED_RELATIVE_FILE is not set")?;
  validate_harpoon_entry(&rel)?;

  let path = harpoon_file_path()?;
  ensure_parent_dir(&path)?;

  if read_entries(&path)?.iter().any(|e| e == &rel) {
    return Ok(0); // already in list
  }

  let mut f = fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&path)
    .context("cannot open harpoon file")?;

  writeln!(f, "{rel}").context("cannot write to harpoon file")?;
  Ok(0)
}

fn harpoon_go(n: usize) -> Result<i32> {
  let root = env::var("ZED_WORKTREE_ROOT").context("ZED_WORKTREE_ROOT is not set")?;
  let path = harpoon_file_path()?;
  let entries = read_entries(&path)?;

  if n > entries.len() {
    bail!(
      "slot {n} is empty (harpoon list has {} entries)",
      entries.len()
    );
  }

  let entry = &entries[n - 1];
  validate_harpoon_entry(entry)?;

  let target = PathBuf::from(&root).join(entry);
  open_in_zed(&target)
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn run() -> Result<i32> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    bail!("usage: harpoon <open|add|go> [n]");
  }

  match args[1].as_str() {
    "open" => harpoon_open(),
    "add" => harpoon_add(),
    "go" => {
      if args.len() < 3 {
        bail!("usage: harpoon go <n>");
      }
      match args[2].parse::<usize>() {
        Ok(n) if n >= 1 => harpoon_go(n),
        _ => bail!("invalid slot: {}", args[2]),
      }
    }
    cmd => bail!("unknown command: {cmd}"),
  }
}

fn main() -> ExitCode {
  match run() {
    Ok(status) => exit_code(status),
    Err(e) => {
      eprintln!("{e}");
      ExitCode::FAILURE
    }
  }
}

fn exit_code(status: i32) -> ExitCode {
  u8::try_from(status).map_or_else(
    |_| {
      eprintln!("invalid exit status from Zed: {status}");
      ExitCode::FAILURE
    },
    ExitCode::from,
  )
}

#[cfg(test)]
mod tests {
  use super::validate_harpoon_entry;

  #[test]
  fn accepts_safe_relative_entries() {
    assert!(validate_harpoon_entry("src/main.rs").is_ok());
    assert!(validate_harpoon_entry("folder with spaces/file.rs").is_ok());
  }

  #[test]
  fn rejects_newline_injection() {
    assert!(validate_harpoon_entry("src/main.rs\n../../etc/passwd").is_err());
    assert!(validate_harpoon_entry("src/main.rs\r../../etc/passwd").is_err());
  }

  #[test]
  fn rejects_empty_entries() {
    assert!(validate_harpoon_entry("").is_err());
  }

  #[test]
  fn rejects_paths_outside_worktree() {
    assert!(validate_harpoon_entry("/etc/passwd").is_err());
    assert!(validate_harpoon_entry("C:\\Windows\\system32\\drivers\\etc\\hosts").is_err());
    assert!(validate_harpoon_entry("\\\\server\\share\\file.rs").is_err());
    assert!(validate_harpoon_entry("../outside.rs").is_err());
    assert!(validate_harpoon_entry("src/../../outside.rs").is_err());
  }
}
