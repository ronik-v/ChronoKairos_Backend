#[derive(Debug)]
pub struct MoexParser {
    ticker: String,
    date_start: String,
    date_end: String
}

impl MoexParser {
    pub fn new(ticker: String, date_start: String, date_end: String) -> Self {
        Self { ticker, date_start, date_end }
    }

    fn make_request_to_moex(&self) {}

    pub fn get(&self) {}
}