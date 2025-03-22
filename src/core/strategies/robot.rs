use crate::core::strategies::models::interface::Model;

#[derive(Debug)]
pub struct TradeRobot<T: Model> {
    token: String,
    model: T,
    limit: f64,
    stop_time: String
}

impl <T: Model> TradeRobot<T> {
    pub fn new(token: String, model: T, limit: f64, stop_time: String) -> Self {
        Self { token, model, limit, stop_time }
    }
}