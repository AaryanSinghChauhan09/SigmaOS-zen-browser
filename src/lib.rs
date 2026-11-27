#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

/// Errors returned by the Zen Browser integration.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ZenError {
    #[error("workspace name cannot be empty")]
    EmptyWorkspaceName,

    #[error("duplicate workspace id '{0}'")]
    DuplicateWorkspaceId(String),

    #[error("invalid pinned URL '{0}': must start with http:// or https://")]
    InvalidPinnedUrl(String),

    #[error("JSON serialization error: {0}")]
    JsonError(String),
}

/// Sidebar placement in Zen Browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SidebarPosition {
    Left,
    Right,
}

/// Theme configuration for Zen Browser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenTheme {
    pub accent_color: String,
    pub compact_mode: bool,
    pub sidebar_position: SidebarPosition,
    pub transparency: bool,
}

impl Default for ZenTheme {
    fn default() -> Self {
        Self {
            accent_color: "#7aa2f7".to_string(),
            compact_mode: true,
            sidebar_position: SidebarPosition::Left,
            transparency: true,
        }
    }
}

/// A Zen Browser workspace with pinned tabs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenWorkspace {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub pinned_urls: Vec<String>,
}

/// Complete Zen Browser profile configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenProfile {
    pub profile_name: String,
    pub split_view_enabled: bool,
    pub theme: ZenTheme,
    pub workspaces: Vec<ZenWorkspace>,
}

impl Default for ZenProfile {
    fn default() -> Self {
        Self {
            profile_name: "sigma-default".to_string(),
            split_view_enabled: true,
            theme: ZenTheme::default(),
            workspaces: vec![
                ZenWorkspace {
                    id: "ws-dev".to_string(),
                    name: "Development".to_string(),
                    icon: "code".to_string(),
                    pinned_urls: vec!["https://docs.rs".to_string()],
                },
                ZenWorkspace {
                    id: "ws-research".to_string(),
                    name: "Research".to_string(),
                    icon: "book".to_string(),
                    pinned_urls: vec!["https://crates.io".to_string()],
                },
            ],
        }
    }
}

impl ZenProfile {
    /// Validates workspaces and pinned URLs.
    pub fn validate(&self) -> Result<(), ZenError> {
        let mut seen_ids = HashSet::new();
        for ws in &self.workspaces {
            if ws.name.trim().is_empty() {
                return Err(ZenError::EmptyWorkspaceName);
            }
            if !seen_ids.insert(ws.id.clone()) {
                return Err(ZenError::DuplicateWorkspaceId(ws.id.clone()));
            }
            for url in &ws.pinned_urls {
                if !(url.starts_with("http://") || url.starts_with("https://")) {
                    return Err(ZenError::InvalidPinnedUrl(url.clone()));
                }
            }
        }
        Ok(())
    }

    /// Exports validated workspace configuration as JSON.
    pub fn export_workspaces_json(&self) -> Result<String, ZenError> {
        self.validate()?;
        serde_json::to_string_pretty(&self.workspaces)
            .map_err(|e| ZenError::JsonError(e.to_string()))
    }

    /// Builds CLI launch arguments for Zen Browser.
    pub fn build_launch_args(&self) -> Vec<String> {
        let mut args = vec!["-P".to_string(), self.profile_name.clone()];
        if self.theme.compact_mode {
            args.push("--zen-compact".to_string());
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_workspaces_json_aaa() {
        // Arrange
        let profile = ZenProfile::default();

        // Act
        let json = profile.export_workspaces_json().expect("export json");

        // Assert
        assert!(json.contains("\"ws-dev\""));
        assert!(json.contains("\"https://docs.rs\""));
    }

    #[test]
    fn test_duplicate_workspace_id_rejected_aaa() {
        // Arrange
        let mut profile = ZenProfile::default();
        profile.workspaces.push(ZenWorkspace {
            id: "ws-dev".to_string(),
            name: "Duplicate".to_string(),
            icon: "star".to_string(),
            pinned_urls: vec![],
        });

        // Act
        let result = profile.validate();

        // Assert
        assert_eq!(result, Err(ZenError::DuplicateWorkspaceId("ws-dev".to_string())));
    }

    #[test]
    fn test_invalid_pinned_url_rejected_aaa() {
        // Arrange
        let mut profile = ZenProfile::default();
        profile.workspaces[0].pinned_urls.push("ftp://bad.example".to_string());

        // Act
        let result = profile.validate();

        // Assert
        assert_eq!(
            result,
            Err(ZenError::InvalidPinnedUrl("ftp://bad.example".to_string()))
        );
    }
}
