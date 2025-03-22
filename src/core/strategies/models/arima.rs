use crate::core::data::moex::models::TickerData;
use crate::core::strategies::models::interface::Model;

struct Arima { ticker_data: TickerData }

impl Model for Arima {
    fn new(ticker_data: TickerData) -> Self { Self { ticker_data } }
}