use alert::Alert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    #[serde(rename = "aliasColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_colors: Option<AliasColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<Alert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "dashLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fill_gradient")]
    pub fill_gradient: Option<u32>,
    #[serde(rename = "gridPos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_pos: Option<GridPos>,
    #[serde(rename = "hiddenSeries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_series: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<Legend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<bool>,
    #[serde(rename = "lineWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<u32>,
    #[serde(rename = "nullPointMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_point_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<DataLink>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointradius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
    #[serde(rename = "seriesOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_overrides: Option<Vec<SeriesOverride>>,
    #[serde(rename = "spaceLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<bool>,
    #[serde(rename = "steppedLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "timeFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO unknown type, might not be string
    pub time_from: Option<String>,
    #[serde(rename = "timeRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO unknown type, might not be string
    pub time_regions: Option<Vec<TimeRegion>>,
    #[serde(rename = "timeShift")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO unknown type, might not be string
    pub time_shift: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<Vec<Threshold>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xaxis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaxes: Option<Vec<Axe>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaxis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<ToolTip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolTip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<u32>,
    #[serde(rename = "valueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRegion {
    // TODO add fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasColors {
    // TODO add fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<bool>,
    #[serde(rename = "alignLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_level: Option<String>, // TODO correct type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    // nullable
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<AxisValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisValue {
    // TODO fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axe {
    #[serde(rename = "$$hashKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "logBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_base: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    // TODO fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "legendFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_format: Option<String>,
    #[serde(rename = "refId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesOverride {
    // TODO fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLink {
    // TODO fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<bool>,
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
        pub evaluator: Option<Evaluator>,
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
    pub struct Evaluator {
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
