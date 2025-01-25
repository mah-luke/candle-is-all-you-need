use candle_core::{Result, Tensor};
use candle_nn::{linear, ops::log_softmax, Linear, Module, VarBuilder};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ProjectionLayer(Linear);

impl ProjectionLayer {
    pub fn new(vb: VarBuilder, config: &Config, vocab_size: usize) -> Result<Self> {
        Ok(Self(linear(
            config.d_model,
            vocab_size,
            vb.pp("projection"),
        )?))
    }

    pub fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let probs = self.0.forward(xs)?;
        log_softmax(&probs, probs.dims().len())
    }
}
