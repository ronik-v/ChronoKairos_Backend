use crate::core::data::moex::models::TickerData;
use crate::core::strategies::models::interface::Model;

struct Sma { ticker_data: TickerData }

impl Model for Sma {
    fn new(ticker_data: TickerData) -> Self { Self { ticker_data } }
}