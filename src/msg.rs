use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

// 実行メッセージの定義
// コントラクトの状態を変更するためのメッセージ
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

// クエリメッセージの定義
// コントラクトの状態を読み取るためのメッセージ
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCount {},
}

// クエリレスポンスの定義
// クエリの結果として返される構造体
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}