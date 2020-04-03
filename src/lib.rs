//! Serializable structures for grafana dashboard import/export
//!
//! # Known bugs / limitations
//! - Missing fields
//! - Too many optional fields
//! - No data validation
//! - No test or coverage
//!
//! Feel free to submit PR's to fix.

use serde::{Deserialize, Serialize};
pub mod annotation;
pub mod link;
pub mod panel;
pub mod templating;

use annotation::Annotations;
use link::Link;
use panel::Panel;
use templating::Templating;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(rename = "gnetId")]
    pub gnet_id: Option<u32>,
    #[serde(rename = "graphTooltip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_tooltip: Option<u32>,
    #[serde(rename = "hideControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_controls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panels: Option<Vec<Panel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templating: Option<Templating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timepicker: Option<TimePicker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    pub variables: Vec<Variable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    // TODO fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePicker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_intervals: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

#[cfg(test)]
mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::collapse;
    use pretty_assertions::assert_eq;
    use serde_json;
    // use utils;

    #[test]
    pub fn test_x() {
        let json = collapse(r##""##);
        let structure: Panel = serde_json::from_str(json.as_str()).expect("failed to stringify");
        assert_eq!(
            json,
            serde_json::to_string(&structure).expect("failed to stringify structure")
        );
    }
}
