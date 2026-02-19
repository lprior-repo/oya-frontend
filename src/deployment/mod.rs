use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use super::twin_runtime::load_twin_definition;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwinDeployment {
    pub name: String,
    pub definition_path: PathBuf,
    pub port: u16,
    pub config: HashMap<String, String>,
    pub pid: Option<u32>,
    pub status: TwinStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TwinStatus {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "error")]
    Error,
}

pub struct TwinDeploymentManager {
    base_path: PathBuf,
    deployments: Arc<Mutex<HashMap<String, TwinDeployment>>>,
}

impl TwinDeploymentManager {
    pub fn new(base_path: &Path) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
            deployments: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn deploy_universe(
        &self,
        manifest_path: &Path,
    ) -> Result<UniverseDeployment, Box<dyn std::error::Error>> {
        let manifest_content = fs::read_to_string(manifest_path)?;
        let yaml: serde_yaml::Value = serde_yaml::from_str(&manifest_content)?;

        let universe_name = yaml["universe"]["name"].as_str().ok_or("unknown")?;
        let twins_config = yaml["universe"]["twins"].as_mapping().ok_or("no twins")?;

        let mut deployments = HashMap::new();
        let mut deployment_result = UniverseDeployment {
            name: universe_name.to_string(),
            twins: Vec::new(),
            status: DeploymentStatus::Deploying,
        };

        let mut port = 9000;
        for (name, config) in twins_config {
            let twin_name = name.as_str().ok_or("unnamed")?;
            let twin_def = config["twin"].as_str().ok_or("")?;

            let twin_path = self.base_path.join("twins").join(twin_def);
            let definition_path = twin_path.join("definition.yaml");

            let deployment = TwinDeployment {
                name: twin_name.to_string(),
                definition_path: definition_path.clone(),
                port,
                config: self.extract_config(config)?,
                pid: None,
                status: TwinStatus::Stopped,
            };

            deployments.insert(twin_name.to_string(), deployment.clone());
            deployment_result.twins.push(deployment);
            port += 1;
        }

        {
            let mut guard = self
                .deployments
                .lock()
                .map_err(|e| format!("Lock error: {}", e))?;
            *guard = deployments;

            for deployment in guard.values_mut() {
                self.start_twin(deployment)?;
                deployment.status = TwinStatus::Running;
            }
        }

        deployment_result.status = DeploymentStatus::Running;
        Ok(deployment_result)
    }

    fn start_twin(
        &self,
        deployment: &mut TwinDeployment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _definition = load_twin_definition(deployment.definition_path.to_str().unwrap())?;

        let child = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("twin-server")
            .arg("--port")
            .arg(deployment.port.to_string())
            .arg("--twin")
            .arg(deployment.definition_path.to_str().unwrap())
            .current_dir(&self.base_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        deployment.pid = Some(child.id());
        deployment.status = TwinStatus::Running;
        Ok(())
    }

    pub fn stop_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self
            .deployments
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        for deployment in guard.values_mut() {
            deployment.status = TwinStatus::Stopped;
        }
        Ok(())
    }

    pub fn get_status(&self, name: &str) -> Option<TwinStatus> {
        let guard = self.deployments.lock().ok()?;
        guard.get(name).map(|d| d.status.clone())
    }

    pub fn get_all_status(&self) -> HashMap<String, TwinStatus> {
        if let Ok(guard) = self.deployments.lock() {
            guard
                .iter()
                .map(|(n, d)| (n.clone(), d.status.clone()))
                .collect()
        } else {
            HashMap::new()
        }
    }

    fn extract_config(
        &self,
        config: &serde_yaml::Value,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut result = HashMap::new();
        if let Some(map) = config.as_mapping() {
            for (k, v) in map {
                if let (Some(ks), Some(vs)) = (k.as_str(), v.as_str()) {
                    result.insert(ks.to_string(), vs.to_string());
                }
            }
        }
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniverseDeployment {
    pub name: String,
    pub twins: Vec<TwinDeployment>,
    pub status: DeploymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    #[serde(rename = "deploying")]
    Deploying,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "error")]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status() {
        assert_ne!(TwinStatus::Stopped, TwinStatus::Running);
    }
}
