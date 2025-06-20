// Shared structs/enums for API and processing

#[derive(Debug)]
pub struct TimeSeriesPoint {
    pub date: String,
    pub value: f64,
}
