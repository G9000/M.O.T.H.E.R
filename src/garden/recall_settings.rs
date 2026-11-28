use super::{
    GardenStore, Result, V1_META, Value, View, contracts, invalid, metadata, put, record,
    unavailable,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Settings {
    aliases: BTreeMap<String, String>,
    keepsakes: BTreeSet<String>,
    event_dates: BTreeMap<String, String>,
}

pub(super) fn words(text: &str) -> String {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn name(raw: &str) -> Result<String> {
    let normalized = words(raw);
    if raw.len() > 120 || normalized.len() > 120 || normalized.is_empty() {
        return Err(invalid(
            "Use a project name of 1 to 120 bytes containing letters or digits.",
        ));
    }
    Ok(normalized)
}

fn valid_date(date: &str) -> bool {
    date.len() == 10
        && chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok_and(|d| d.to_string() == date)
}

impl Settings {
    pub(super) fn load(tx: &impl View) -> Result<Self> {
        Ok(tx
            .get(V1_META, "recallSettings")?
            .map(serde_json::from_value)
            .transpose()?
            .unwrap_or_default())
    }
    pub(super) fn validate(&self, tx: &impl View) -> Result<()> {
        for (alias, project) in &self.aliases {
            if name(alias)? != *alias
                || name(project)? != *project
                || self.aliases.contains_key(project)
            {
                return Err(invalid(
                    "Aliases must point directly to a canonical project.",
                ));
            }
        }
        for id in self.keepsakes.iter().chain(self.event_dates.keys()) {
            contracts::id(id)?;
            record(tx, id)?;
        }
        if self.event_dates.values().any(|d| !valid_date(d)) {
            return Err(invalid("Use a calendar date in YYYY-MM-DD format."));
        }
        Ok(())
    }
    pub(super) fn canonical<'a>(&'a self, name: &'a str) -> &'a str {
        self.aliases.get(name).map_or(name, String::as_str)
    }
    pub(super) fn names<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let canonical = self.canonical(name);
        std::iter::once(canonical)
            .chain(
                self.aliases
                    .iter()
                    .filter(|(_, v)| v.as_str() == canonical)
                    .map(|(k, _)| k.as_str()),
            )
            .collect()
    }
    pub(super) fn hidden(&self, id: &str) -> bool {
        self.keepsakes.contains(id)
    }
    pub(super) fn date(&self, id: &str) -> Option<Value> {
        self.event_dates
            .get(id)
            .map(|d| json!({"value":d,"source":"owner"}))
    }
}

pub(super) fn project_match<'a>(r: &Value, names: &[&'a str]) -> Option<&'a str> {
    for key in ["title", "content"] {
        let normalized = format!(" {} ", words(r[key].as_str().unwrap_or_default()));
        if let Some(name) = names
            .iter()
            .find(|n| normalized.contains(&format!(" {n} ")))
        {
            return Some(*name);
        }
    }
    None
}

impl GardenStore {
    fn edit_recall(
        &self,
        edit: impl FnOnce(&mut Settings, &redb::WriteTransaction) -> Result<()>,
    ) -> Result<Value> {
        let tx = self.db.begin_write().map_err(unavailable)?;
        metadata(&tx, "workspace")?;
        let mut settings = Settings::load(&tx)?;
        edit(&mut settings, &tx)?;
        let value = serde_json::to_value(&settings)?;
        if serde_json::to_vec(&value)?.len() > 1_048_576 {
            return Err(invalid("Recall settings exceed the workspace limit."));
        }
        put(&tx, V1_META, "recallSettings", &value)?;
        tx.commit().map_err(unavailable)?;
        Ok(json!({"saved":true}))
    }

    /// Register or remove an explicit former project name.
    pub fn project_alias(&self, project: &str, alias: &str, remove: bool) -> Result<Value> {
        let project = name(project)?;
        let alias = name(alias)?;
        self.edit_recall(|s, _| {
            if s.aliases.get(&alias).is_some_and(|p| *p != project) {
                return Err(invalid("This alias belongs to another project."));
            }
            if remove {
                s.aliases.remove(&alias);
            } else {
                if alias == project
                    || s.aliases.contains_key(&project)
                    || s.aliases.values().any(|v| *v == alias)
                {
                    return Err(invalid(
                        "Aliases must point directly to a distinct canonical project.",
                    ));
                }
                s.aliases.insert(alias, project);
            }
            Ok(())
        })
    }
    /// Choose whether an existing record may appear in ordinary recall.
    pub fn set_keepsake(&self, id: &str, enabled: bool) -> Result<Value> {
        self.edit_recall(|s, tx| {
            record(tx, id)?;
            if enabled {
                s.keepsakes.insert(id.into());
            } else {
                s.keepsakes.remove(id);
            }
            Ok(())
        })
    }
    /// Set or clear the owner-supplied calendar date of an event.
    pub fn set_event_date(&self, id: &str, date: Option<&str>) -> Result<Value> {
        self.edit_recall(|s, tx| {
            record(tx, id)?;
            if let Some(date) = date {
                if !valid_date(date) {
                    return Err(invalid("Use a calendar date in YYYY-MM-DD format."));
                }
                s.event_dates.insert(id.into(), date.into());
            } else {
                s.event_dates.remove(id);
            }
            Ok(())
        })
    }
}
