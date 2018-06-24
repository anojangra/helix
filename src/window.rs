use schemas::Quote;

#[derive(Debug, Clone)]
pub struct Window {
    pub window: Vec<Quote>,
    pub current_quote: Quote,
}

impl Window {
    pub fn current_diff(&self) -> f32 {
        let end_idx = self.window.len();
        let end_window_quote = &self.window[end_idx.clone()];
        self.current_quote.close - end_window_quote.close
    }

    // Takes the lagged window of quotes and the current window and creates a
    // a single vector of quote
    pub fn flatten(&self) -> Vec<Quote> {
        let mut w = self.window.clone();
        w.push(self.current_quote.clone());
        w
    }
}