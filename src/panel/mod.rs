use alert::Alert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<Alert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "gridPos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_pos: Option<GridPos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridPos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
}

pub mod alert {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Alert {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "alertRuleTag")]
        pub alert_rule_tag: Option<RuleTag>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub conditions: Option<Vec<Condition>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleTag {
        // TODO define properties
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Condition {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub evaluator: Option<Evalurator>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operator: Option<Operator>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub query: Option<Query>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reducer: Option<Reducer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Evalurator {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Operator {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Query {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<u32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Reducer {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }
}
