//! Explicit per-user Windows sign-in integration.
use super::{Result, Startup, failure};
#[cfg(windows)]
use super::{Settings, load_settings, quoted_path, root};
#[cfg(windows)]
use std::{fs, io::Write, path::PathBuf};
#[cfg(windows)]
const MARKER: &str = "' MOTHER owned ChatGPT startup v1\r\n";
#[cfg(windows)]
fn path() -> Result<PathBuf> {
    let app = std::env::var_os("APPDATA")
        .ok_or_else(|| failure("Windows startup folder is unavailable."))?;
    let home = root()?;
    use sha2::{Digest, Sha256};
    let id = format!("{:x}", Sha256::digest(home.to_string_lossy().as_bytes()));
    Ok(PathBuf::from(app)
        .join("Microsoft/Windows/Start Menu/Programs/Startup")
        .join(format!("MOTHER-ChatGPT-{}.vbs", &id[..16])))
}
#[cfg(windows)]
fn contents(settings: &Settings) -> Result<String> {
    let home = mother::config::data_directory()?;
    quoted_path(&home)?;
    let cmd =
        format!("{} connect chatgpt", quoted_path(&settings.mother_exe)?).replace('"', "\"\"");
    Ok(format!(
        "{MARKER}Set shell = CreateObject(\"WScript.Shell\")\r\nshell.Environment(\"PROCESS\")(\"MOTHER_HOME\") = \"{}\"\r\nshell.Run \"{cmd}\", 0, False\r\n",
        home.to_string_lossy()
    ))
}
#[cfg(windows)]
fn validate_executable(settings: &Settings) -> Result<()> {
    if settings.mother_exe.components().any(|c| {
        c.as_os_str()
            .to_string_lossy()
            .eq_ignore_ascii_case("target")
    }) {
        return Err(failure(
            "Install M.O.T.H.E.R at a stable path before enabling sign-in startup.",
        ));
    }
    if !settings.mother_exe.is_file() {
        return Err(failure("Stable M.O.T.H.E.R executable is missing."));
    }
    Ok(())
}
#[cfg(windows)]
fn existing(path: &std::path::Path) -> Result<Option<String>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(m) if m.is_file() && !m.file_type().is_symlink() && m.len() < 16384 => {}
        _ => {
            return Err(failure(
                "Startup path is not an owned ordinary file; it was preserved.",
            ));
        }
    }
    let s = fs::read_to_string(path).map_err(|_| failure("Cannot inspect startup launcher."))?;
    if !s.starts_with(MARKER) {
        return Err(failure(
            "Startup launcher is not owned by M.O.T.H.E.R; it was preserved.",
        ));
    }
    Ok(Some(s))
}
pub(super) fn enabled() -> Result<bool> {
    #[cfg(windows)]
    {
        let Some(settings) = load_settings()? else {
            return Ok(false);
        };
        let Some(prior) = existing(&path()?)? else {
            return Ok(false);
        };
        let expected = contents(&settings)?;
        if prior == expected {
            Ok(true)
        } else {
            Err(failure(
                "Startup launcher differs from saved settings; inspect it before changing startup.",
            ))
        }
    }
    #[cfg(not(windows))]
    {
        Ok(false)
    }
}
pub(super) fn set(value: Startup) -> Result<()> {
    #[cfg(windows)]
    {
        let settings = load_settings()?
            .ok_or_else(|| failure("Complete connection setup before configuring startup."))?;
        let target = path()?;
        let prior = existing(&target)?;
        let expected = contents(&settings)?;
        if prior.as_ref().is_some_and(|s| s != &expected) {
            return Err(failure(
                "Startup launcher differs from saved settings; it was preserved.",
            ));
        }
        match value {
            Startup::Disabled => {
                if prior.is_some() {
                    fs::remove_file(target)
                        .map_err(|_| failure("Cannot remove owned startup launcher."))?;
                }
            }
            Startup::Enabled => {
                validate_executable(&settings)?;
                super::verify_grant(&settings)?;
                if super::secret(&settings)?.is_none() {
                    return Err(failure(
                        "Save a runtime key in the OS credential store before enabling startup.",
                    ));
                }
                if prior.is_none() {
                    let mut file = mother::config::create_private_file(&target)
                        .map_err(|_| failure("Cannot exclusively create startup launcher."))?;
                    file.write_all(expected.as_bytes())?;
                    file.sync_all()?;
                }
            }
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = value;
        Err(failure(
            "Sign-in startup integration is available only on Windows.",
        ))
    }
}
#[cfg(all(test, windows))]
mod tests {
    use super::*;
    #[test]
    fn startup_rejects_unstable_executable() {
        let s = Settings {
            version: 1,
            tunnel_id: "tunnel_test".into(),
            organization_id: None,
            client_config: "C:/client.json".into(),
            tunnel_client: "C:/tunnel.exe".into(),
            mother_exe: "C:/repo/target/release/mother.exe".into(),
        };
        assert!(validate_executable(&s).is_err());
        assert!(contents(&s).is_ok());
    }
}
