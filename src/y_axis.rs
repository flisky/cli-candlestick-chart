use std::{cell::RefCell, rc::Rc};

use crate::chart_data::ChartData;

pub struct YAxis {
    pub chart_data: Rc<RefCell<ChartData>>,
}

impl YAxis {
    pub const CHAR_PRECISION: i64 = 4;
    pub const DEC_PRECISION: i64 = 2;
    pub const MARGIN_RIGHT: i64 = 4;
    pub const PRECISION: i64 = 2;

    pub const WIDTH: i64 = YAxis::CHAR_PRECISION
        + YAxis::MARGIN_RIGHT
        + 1
        + YAxis::DEC_PRECISION
        + YAxis::MARGIN_RIGHT;

    pub fn new(chart_data: Rc<RefCell<ChartData>>) -> YAxis {
        YAxis { chart_data }
    }

    pub fn price_to_height(&self, price: f64) -> f64 {
        let chart_data = self.chart_data.borrow();
        let height = chart_data.height;

        let min_value = chart_data.visible_candle_set.min_value;
        let max_value = chart_data.visible_candle_set.max_value;

        (price - min_value) / (max_value - min_value) * height as f64
    }

    pub fn render_line(&self, y: u16) -> String {
        match y % 4 {
            0 => self.render_tick(y),
            _ => self.render_empty(),
        }
    }

    fn render_tick(&self, y: u16) -> String {
        let chart_data = self.chart_data.borrow();
        let min_value = chart_data.visible_candle_set.min_value;
        let max_value = chart_data.visible_candle_set.max_value;
        let height = chart_data.height;

        let price = min_value + (y as f64 * (max_value - min_value) / height as f64);
        let cell_min_length = (4 + 3) as usize;

        format!(
            "{0:<cell_min_length$.2} │┈{margin}",
            price,
            cell_min_length = cell_min_length,
            margin = " ".repeat(YAxis::MARGIN_RIGHT as usize)
        )
    }

    fn render_empty(&self) -> String {
        let cell = " ".repeat((4 + 4) as usize);
        let margin = " ".repeat((YAxis::MARGIN_RIGHT + 1).try_into().unwrap());

        format!("{}│{}", cell, margin)
    }
}
