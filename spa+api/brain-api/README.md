# Brain API (Refined)

## Overview

- On startup, the API fetches data from:
  `https://brains.tradingeconomics.com/v2/search/comtrade?q=ukraine&pp=50&p=0`
- Only the following fields are kept for each record: `country`, `category`, `currency`, `name`, `type`
- All endpoints serve data from this in-memory dataset using **Polars** for analytics
- No further calls to the brain API are made after startup

## How Polars is Used

- **Data Loading:** The fetched JSON is loaded into a Polars DataFrame for fast, in-memory analytics.
- **Pie Chart Data:** Polars groups by `category` and counts records for pie charting.
- **Unique Import/Export Names:** Polars filters by `category` and selects unique `name` values.
- **Raw Data Serialization:** The DataFrame is iterated row-by-row and converted to JSON for the `/api/raw` endpoint.
- **All analytics and filtering are performed with Polars, making the backend fast and scalable.**

## Data Model

Each record has:
```json
{
  "country": "Ukraine",
  "category": "Imports",
  "currency": "USD",
  "name": "Ukraine Imports by Category",
  "type": "comtrade"
}
```

## API Endpoints

### 1. Status
**GET `/api/status`**

Returns the status of the in-memory dataset.

**Response:**
```json
{
  "data_loaded": true,
  "total": 50,
  "last_updated": "2025-06-20T10:59:11.566457+00:00"
}
```

### 2. Raw Data
**GET `/api/raw`**

Returns the full filtered dataset as JSON.

**Response:**
```json
[
  { "country": "Ukraine", "category": "Imports", ... },
  ...
]
```

### 3. Pie Chart Data
**GET `/api/pie`**

Returns counts of records grouped by `category` (e.g., Imports/Exports) for pie charting.

**Response:**
```json
[
  { "label": "Imports", "count": 30 },
  { "label": "Exports", "count": 20 }
]
```

### 4. Unique Import Names
**GET `/api/import-names`**

Returns a list of unique `name` values where `category` is "Imports".

**Response:**
```json
[
  "Ukraine Imports by Category",
  "United States Imports from Ukraine of Dairy products, eggs, honey, edible products",
  ...
]
```

### 5. Unique Export Names
**GET `/api/export-names`**

Returns a list of unique `name` values where `category` is "Exports".

**Response:**
```json
[
  "Ukraine Exports by Category",
  "United States exports of live animals to Ukraine",
  ...
]
```

---

## Example curl Commands

```bash
# Check API status
curl http://localhost:3002/api/status

# Get all filtered raw data
curl http://localhost:3002/api/raw

# Get pie chart data (import/export counts)
curl http://localhost:3002/api/pie

# Get unique import names
curl http://localhost:3002/api/import-names

# Get unique export names
curl http://localhost:3002/api/export-names
```

---

## Notes
- All endpoints are fast and local after the initial data fetch.
- If you want to change the data source or fields, update the startup fetch and filtering logic in the backend.

## .env
```
BASE_URL=https://brains.tradingeconomics.com
```

## How to Use
- Run with `cargo run`
- The server will start on `http://localhost:3002` (default)

## Endpoints

### `/api/search` (GET)
- **Main endpoint for the SPA to fetch processed, ready-to-visualize data.**
- Returns: summary analytics and a sample record.
- SPA should use this for charts, tables, and dashboards.

### `/api/imports/by-country` (GET)
- Returns: Array of `{ country, value, year }` for Ukraine imports, grouped and summed by country.
- **Intended for bar/treemap charts of imports by country.**
- Example response:
  ```json
  [
    { "country": "China", "value": 17060000000, "year": 2024 },
    { "country": "Poland", "value": 8000000000, "year": 2024 },
    ...
  ]
  ```

### `/api/imports/by-category` (GET)
- Returns: Array of `{ category, value, year }` for Ukraine imports, grouped and summed by category.
- **Intended for bar/pie/treemap charts of imports by category.**
- Example response:
  ```json
  [
    { "category": "Mineral fuels, oils", "value": 5000000000, "year": 2024 },
    { "category": "Electrical equipment", "value": 3000000000, "year": 2024 },
    ...
  ]
  ```

### `/api/raw` (GET)
- Returns the raw hits array as fetched from the brain API.
- Useful for debugging or advanced users.

### `/api/status` (GET)
- Returns the current data lifecycle status as JSON:
  ```json
  {
    "fetched": true,
    "ready": true,
    "in_use": false,
    "last_updated": "2024-06-20T12:34:56Z"
  }
  ```
- SPA should poll this endpoint to update a UI gauge/status bar (e.g., "data fetched", "data ready", "data in use").

### `/api/in-use` (POST)
- SPA can POST to this endpoint to notify the backend that the data is now being used (for logging/monitoring).
- Returns a simple status JSON.

### 1. Data Status

**GET `/api/status`**

Returns the data lifecycle status.

**Response:**
```json
{
  "status": "ready",
  "last_updated": "2024-06-22T12:00:00Z"
}
```

---

### 2. Country List

**GET `/api/countries`**

Returns a list of all unique countries in the dataset.

**Response:**
```json
["Germany", "Ukraine", "Mexico", ...]
```

---

### 3. Country Details

**GET `/api/country/{country}`**

Returns all indicators for the country, and total number of unique categories under Import and Export.

**Response:**
```json
{
  "import_category_count": 5,
  "export_category_count": 3,
  "indicators": [
    {
      "type": "Import",
      "category": "Machinery",
      "frequency": "Annual",
      "title": "Germany Imports of Machinery"
      // ... other fields
    },
    ...
  ]
}
```

---

**GET `/api/country/{country}/summary`**

Returns summary counts for the country.

**Response:**
```json
{
  "import_count": 12,
  "export_count": 8,
  "import_category_count": 5,
  "export_category_count": 3
}
```

---

**GET `/api/country/{country}/charts`**

Returns data for pie/bar charts.

**Response:**
```json
{
  "pie": [
    { "label": "Import", "count": 12 },
    { "label": "Export", "count": 8 }
  ],
  "bar": [
    { "category": "Machinery", "import_count": 5, "export_count": 2 },
    { "category": "Agriculture", "import_count": 3, "export_count": 1 }
  ]
}
```

---

## Example curl Commands

```bash
# Check status
curl http://localhost:3002/api/status

# Get country list
curl http://localhost:3002/api/countries

# Get all indicators and category counts for Germany
curl http://localhost:3002/api/country/Germany

# Get summary counts for Germany
curl http://localhost:3002/api/country/Germany/summary

# Get chart data for Germany
curl http://localhost:3002/api/country/Germany/charts
```

---

## Data Lifecycle Flow
- **Data Fetched:** Raw data has been pulled from the brain API (backend prefetch, not user-triggered).
- **Data Ready:** Backend has finished preprocessing/feature engineering (Polars), and the data is ready for the SPA.
- **Data In Use:** SPA is actively consuming/visualizing the data.
- The SPA should use `/api/status` to monitor these states and `/api/search` or the new endpoints to fetch the data for visualization.

---

## Features
- No API key required
- Uses Polars for grouping, summing, sorting, and analytics
- Robust error handling
- Data lifecycle status tracking for UI integration
- Endpoints for ready-to-visualize data by country and category
