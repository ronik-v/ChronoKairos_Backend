use crate::core::data::moex::models::TickerData;

pub trait Model {
    fn new(ticker_data: TickerData) -> Self;
    // TODO: add all func
}