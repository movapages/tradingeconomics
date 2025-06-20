//! Brain API for Trading Economics
//!
//! Proxies and processes data from the Trading Economics brain endpoint.
//! Uses Polars for analytics. No API key required.
//!
//! Endpoints:
//! - GET /api/search (summary by country)
//! - GET /api/raw (raw hits)
//! - GET /api/status (data lifecycle status)
//! - POST /api/in-use (mark data as in use)
//! - GET /api/imports/by-country (NEW: imports grouped by country)
//! - GET /api/imports/by-category (NEW: imports grouped by category)

use axum::{routing::get, Router, response::IntoResponse, Json};
use dotenvy::dotenv;
use std::env;
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};
use polars::prelude::*;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use chrono::Utc;
use std::io::Cursor;

#[derive(Clone, Debug, serde::Serialize)]
struct DataStatus {
    fetched: bool,
    ready: bool,
    in_use: bool,
    last_updated: String,
}

static STATUS: Lazy<Arc<Mutex<DataStatus>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DataStatus {
        fetched: false,
        ready: false,
        in_use: false,
        last_updated: Utc::now().to_rfc3339(),
    }))
});

fn update_status(fetched: Option<bool>, ready: Option<bool>, in_use: Option<bool>) {
    let mut status = STATUS.lock().unwrap();
    if let Some(f) = fetched { status.fetched = f; }
    if let Some(r) = ready { status.ready = r; }
    if let Some(i) = in_use { status.in_use = i; }
    status.last_updated = Utc::now().to_rfc3339();
}

static DATA: Lazy<Arc<Mutex<Option<DataFrame>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

async fn load_data() -> Result<DataFrame, String> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "https://brains.tradingeconomics.com".to_string());
    let url = format!("{}/v2/search/comtrade?q=ukraine&pp=50&p=0", base_url);
    let resp = reqwest::get(&url).await.map_err(|e| format!("Request error: {}", e))?;
    let text = resp.text().await.map_err(|e| format!("Read error: {}", e))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("JSON parse error: {}", e))?;
    let hits = v.get("hits").and_then(|h| h.as_array()).cloned().unwrap_or_default();
    // Only keep the required fields
    let filtered: Vec<_> = hits.into_iter().map(|rec| {
        json!({
            "country": rec.get("country").cloned().unwrap_or(json!(null)),
            "category": rec.get("category").cloned().unwrap_or(json!(null)),
            "currency": rec.get("currency").cloned().unwrap_or(json!(null)),
            "name": rec.get("name").cloned().unwrap_or(json!(null)),
            "type": rec.get("type").cloned().unwrap_or(json!(null)),
        })
    }).collect();
    let json_string = serde_json::to_string(&filtered).map_err(|e| format!("Serialization error: {}", e))?;
    let df = JsonReader::new(Cursor::new(json_string)).finish().map_err(|e| format!("Polars error: {}", e))?;
    Ok(df)
}

fn get_df() -> Result<DataFrame, String> {
    let data_guard = DATA.lock().unwrap();
    if let Some(ref df) = *data_guard {
        Ok(df.clone())
    } else {
        Err("Data not loaded yet".to_string())
    }
}

async fn status() -> impl IntoResponse {
    let status = STATUS.lock().unwrap().clone();
    let data_guard = DATA.lock().unwrap();
    let data_loaded = data_guard.is_some();
    let total = data_guard.as_ref().map(|df| df.height()).unwrap_or(0);
    Json(json!({
        "data_loaded": data_loaded,
        "total": total,
        "last_updated": status.last_updated
    }))
}

async fn raw() -> impl IntoResponse {
    match get_df() {
        Ok(df) => {
            // Convert DataFrame to Vec<serde_json::Value> row-by-row
            let cols = df.get_column_names();
            let mut rows = Vec::new();
            for i in 0..df.height() {
                let mut map = serde_json::Map::new();
                for col in &cols {
                    let series = df.column(col).unwrap();
                    let val = series.get(i);
                    map.insert(col.to_string(), match val {
                        Ok(v) => match v {
                            AnyValue::Null => serde_json::Value::Null,
                            AnyValue::String(s) => serde_json::Value::String(s.to_string()),
                            AnyValue::Int64(n) => serde_json::Value::from(n),
                            AnyValue::UInt64(n) => serde_json::Value::from(n),
                            AnyValue::Float64(f) => serde_json::Value::from(f),
                            AnyValue::Boolean(b) => serde_json::Value::Bool(b),
                            _ => serde_json::Value::String(v.to_string()),
                        },
                        Err(_) => serde_json::Value::Null,
                    });
                }
                rows.push(serde_json::Value::Object(map));
            }
            Json(json!(rows))
        },
        Err(e) => Json(json!({ "error": e })),
    }
}

async fn pie() -> impl IntoResponse {
    match get_df() {
        Ok(df) => {
            let pie = df
                .lazy()
                .group_by([col("category")])
                .agg([col("name").count().alias("count")])
                .collect();
            match pie {
                Ok(pie_df) => {
                    let mut rows = Vec::new();
                    let cat_col = pie_df.column("category").ok();
                    let count_col = pie_df.column("count").ok();
                    if let (Some(cat_col), Some(count_col)) = (cat_col, count_col) {
                        for i in 0..pie_df.height() {
                            let label = match cat_col.get(i) {
                                Ok(v) => v.to_string(),
                                Err(_) => String::new(),
                            };
                            let count = match count_col.get(i) {
                                Ok(v) => v.extract::<i64>().unwrap_or(0) as u32,
                                Err(_) => 0,
                            };
                            rows.push(json!({"label": label, "count": count}));
                        }
                    }
                    Json(json!(rows))
                },
                Err(e) => Json(json!({"error": format!("Polars error: {}", e)})),
            }
        },
        Err(e) => Json(json!({ "error": e })),
    }
}

async fn import_names() -> impl IntoResponse {
    match get_df() {
        Ok(df) => {
            let imports = df
                .lazy()
                .filter(col("category").eq(lit("Imports")))
                .select([col("name")])
                .unique(None, UniqueKeepStrategy::First)
                .collect();
            match imports {
                Ok(imports_df) => {
                    let col = imports_df.column("name").ok();
                    let names = if let Some(col) = col {
                        if let Ok(str_col) = col.str() {
                            str_col.into_no_null_iter().map(|s| s.to_string()).collect::<Vec<_>>()
                        } else {
                            (0..col.len()).map(|i| match col.get(i) { Ok(v) => v.to_string(), Err(_) => String::new() }).collect::<Vec<_>>()
                        }
                    } else { vec![] };
                    Json(json!(names))
                },
                Err(e) => Json(json!({"error": format!("Polars error: {}", e)})),
            }
        },
        Err(e) => Json(json!({ "error": e })),
    }
}

async fn export_names() -> impl IntoResponse {
    match get_df() {
        Ok(df) => {
            let exports = df
                .lazy()
                .filter(col("category").eq(lit("Exports")))
                .select([col("name")])
                .unique(None, UniqueKeepStrategy::First)
                .collect();
            match exports {
                Ok(exports_df) => {
                    let col = exports_df.column("name").ok();
                    let names = if let Some(col) = col {
                        if let Ok(str_col) = col.str() {
                            str_col.into_no_null_iter().map(|s| s.to_string()).collect::<Vec<_>>()
                        } else {
                            (0..col.len()).map(|i| match col.get(i) { Ok(v) => v.to_string(), Err(_) => String::new() }).collect::<Vec<_>>()
                        }
                    } else { vec![] };
                    Json(json!(names))
                },
                Err(e) => Json(json!({"error": format!("Polars error: {}", e)})),
            }
        },
        Err(e) => Json(json!({ "error": e })),
    }
}

async fn refresh() -> impl IntoResponse {
    match load_data().await {
        Ok(df) => {
            {
                let mut data_guard = DATA.lock().unwrap();
                *data_guard = Some(df);
            }
            update_status(Some(true), Some(true), Some(false));
            let status = STATUS.lock().unwrap().clone();
            let total = DATA.lock().unwrap().as_ref().map(|df| df.height()).unwrap_or(0);
            Json(json!({
                "ok": true,
                "total": total,
                "last_updated": status.last_updated
            }))
        },
        Err(e) => Json(json!({ "ok": false, "error": e })),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    // Load data at startup
    let df = load_data().await;
    {
        let mut data_guard = DATA.lock().unwrap();
        *data_guard = df.ok();
    }
    update_status(Some(true), Some(true), Some(false));

    let app = Router::new()
        .route("/api/status", get(status))
        .route("/api/raw", get(raw))
        .route("/api/pie", get(pie))
        .route("/api/import-names", get(import_names))
        .route("/api/export-names", get(export_names))
        .route("/api/refresh", get(refresh))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
} 