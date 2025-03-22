#[derive(Debug)]
pub struct TickerData {
    pub (crate) ticker: String,
    pub (crate) open: Vec<f64>,
    pub (crate) close: Vec<f64>,
    pub (crate) low: Vec<f64>,
    pub (crate) high: Vec<f64>,
    pub (crate) dates: Vec<String>,
}