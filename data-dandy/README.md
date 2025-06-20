# Data Dandy

## What is it?
Data Dandy is a Rust-based desktop dashboard for financial and economic data exploration, powered by TradingEconomics API, Polars, and egui. It enables users to fetch, preprocess, and visualize time series data (e.g., crude oil prices) with a focus on feature engineering, outlier detection, and trend analysis—all in a modern, interactive GUI.

---

## Motivation
- **Feature engineering** is a crucial step in any data science or ML workflow. Clean, well-processed data leads to better insights and models.
- Financial and economic data is often noisy, incomplete, and full of outliers—making preprocessing essential.
- Most tools for this are Python-based; Data Dandy shows how Rust can be used for high-performance, type-safe, and interactive data analysis.
- The project is a template for building modern Rust data dashboards, and a playground for experimenting with data wrangling, visualization, and ML in Rust.

---

## Toolkit
- **Rust**: Safe, fast, and modern systems language.
- **egui/eframe**: Immediate-mode GUI for desktop dashboards.
- **Polars**: Blazing-fast DataFrame library for data wrangling and feature engineering.
- **reqwest**: HTTP client for API calls.
- **dotenv**: Load API keys and config from `.env`.
- **serde/serde_json**: Data parsing and serialization.
- **plotters**: Charting and visualization.
- **TradingEconomics API**: Real-time and historical economic/financial data.

---

## Architecture / Integral Parts

```
data-dandy/
├── src/
│   ├── main.rs            # App entry point
│   ├── gui.rs             # egui dashboard and UI logic
│   ├── api/
│   │   ├── mod.rs         # API querying logic
│   │   ├── te.rs          # TradingEconomics-specific code
│   │   └── constants.rs   # Endpoints, symbols, etc.
│   ├── processing/
│   │   ├── mod.rs         # Data processing API
│   │   └── polars_utils.rs# Polars-based wrangling, outlier detection
│   └── types.rs           # Shared structs/enums
├── .env                   # API keys and config
└── README.md              # This file
```

- **GUI**: All user interaction, dashboard layout, and visualization.
- **API**: Querying TradingEconomics and other APIs, managing endpoints and keys.
- **Processing**: Data cleaning, feature engineering, outlier/trend detection (Polars).
- **Types**: Shared data structures for API and processing.

---

## How-tos

### 1. Setup
- Install Rust: https://rustup.rs
- Clone this repo
- Add your TradingEconomics API key to `.env`:
  ```
  TE_API_KEY=your_api_key_here
  ```
- Build and run:
  ```sh
  cargo run
  ```

### 2. Fetching Data
- Use the dashboard to select an endpoint (e.g., crude oil prices)
- The app will fetch data from TradingEconomics using your API key

### 3. Data Processing
- Raw data is loaded into a Polars DataFrame
- Outlier detection, trend analysis, and feature engineering are performed
- Results are shown in tables and charts

### 4. Extending the App
- Add new endpoints in `api/te.rs` and `api/constants.rs`
- Add new processing functions in `processing/polars_utils.rs`
- Add new visualizations in `gui.rs`

### 5. Development Guide
- Use this README as your dev guide
- Keep code modular and well-documented
- PRs and issues welcome!

---

## License
MIT 