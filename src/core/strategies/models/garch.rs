use crate::core::data::moex::models::TickerData;
use crate::core::strategies::models::interface::Model;

struct Garch { ticker_data: TickerData }

impl Model for Garch {
    fn new(ticker_data: TickerData) -> Self { Self { ticker_data } }
}