//! Owner-loaded pin eligibility for opt-in session context reads.
use crate::{
    config::{check_private_output, read_json},
    contracts,
    error::{Error, Result, invalid},
};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct File {
    version: u32,
    workspace_id: String,
    profiles: Vec<Profile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    name: String,
    pins: Vec<String>,
}

/// Optional runtime policy. Profiles narrow a context read under its current grant.
#[derive(Debug, Default)]
pub struct RecallProfiles {
    workspace_id: String,
    profiles: BTreeMap<String, BTreeSet<String>>,
}

impl RecallProfiles {
    /// Load a bounded, owner-private policy snapshot. Restart the process to load changes.
    pub fn load(path: &Path) -> Result<Self> {
        let path = std::path::absolute(path)?;
        check_private_output(&path)?;
        let file: File = serde_json::from_value(read_json(&path, 32 * 1024)?)?;
        contracts::id(&file.workspace_id)?;
        if file.version != 1 || file.profiles.len() > 32 {
            return Err(invalid(
                "Unsupported recall profile policy version or profile count.",
            ));
        }
        let mut profiles = BTreeMap::new();
        for profile in file.profiles {
            validate_name(&profile.name)?;
            if profile.pins.len() > crate::pins::MAX_PINS {
                return Err(invalid(
                    "A recall profile can select at most twelve pin IDs.",
                ));
            }
            let mut pins = BTreeSet::new();
            for id in profile.pins {
                contracts::id(&id)?;
                if !pins.insert(id) {
                    return Err(invalid("A recall profile repeats a pin ID."));
                }
            }
            if profiles.insert(profile.name, pins).is_some() {
                return Err(invalid("The recall profile policy repeats a profile name."));
            }
        }
        Ok(Self {
            workspace_id: file.workspace_id,
            profiles,
        })
    }

    pub(crate) fn pins(&self, name: &str, workspace_id: &str) -> Result<&BTreeSet<String>> {
        validate_name(name)?;
        self.profiles.get(name)
            .filter(|_| self.workspace_id == workspace_id)
            .ok_or_else(|| Error::domain("RECALL_PROFILE_UNAVAILABLE",
                "The requested recall profile is unavailable for this workspace. No unprofiled context was returned."))
    }
}

fn validate_name(name: &str) -> Result<()> {
    if !(1..=40).contains(&name.len())
        || !name.as_bytes()[0].is_ascii_lowercase()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(invalid(
            "A recall profile name must start with a lowercase ASCII letter and use up to forty lowercase letters, digits, or hyphens.",
        ));
    }
    Ok(())
}
