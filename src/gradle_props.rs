use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};

pub struct GradleProperties {
    pub file: String,
    pub keys: Vec<String>,
    pub values: HashMap<String, String>,
}

impl GradleProperties {

    //TODO use lifetime

    pub fn load(file: &str) -> Result<GradleProperties> {
        let mut props = GradleProperties {
            file: file.to_string(),
            keys: Vec::new(),
            values: HashMap::new(),
        };
        let content = fs::read_to_string(file)
            .with_context(||format!("Cannot read file '{file}'"))?;
        for line in content.lines() {
            let (key, value) = line
                .split_once("=")
                .unwrap_or(("", ""));
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            if !key.is_empty() {
                props.keys.push(key.to_string()); //TODO optimize
                props.values.insert(key, value);
            }
        }
        return Ok(props)
    }

    pub fn save(&self) -> Result<()> {
        let content = self.keys.iter()
            .map(|key| {
                let value = self.get(key).unwrap_or("");
                format!("{}={}", key, value)
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(&self.file, content)
            .context("Cannot update version")?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        return self.values.get(key)
            .map(|it|it.as_str())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let key = key.to_string();
        let value = value.to_string();
        if !self.keys.contains(&key) {
            self.keys.push(key.clone());
        }
        self.values.insert(key, value);
    }

}