use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;

use crate::attention::multihead::MultiHeadAttentionBlock;
use crate::config::Config;
use crate::feed_forward::FeedForwardBlock;
use crate::residual_connections::{self, ResidualConnection};

pub struct DecoderBlock {
    attention: MultiHeadAttentionBlock,
    cross_attention: MultiHeadAttentionBlock,
    feed_forward: FeedForwardBlock,
    residual_connections: Vec<ResidualConnection>,
}

impl DecoderBlock {
    pub fn new(config: &Config, vb: VarBuilder) -> Result<Self> {
        let mut residual_connections = Vec::with_capacity(2);
        for _ in 0..3 {
            residual_connections.push(ResidualConnection::new(config)?);
        }

        Ok(Self {
            attention: MultiHeadAttentionBlock::new(vb.pp("attention"), config)?,
            cross_attention: MultiHeadAttentionBlock::new(vb.pp("cross_attention"), config)?,
            feed_forward: FeedForwardBlock::new(vb.pp("feed_forward"), config)?,
            residual_connections,
        })
    }

    pub fn forward(
        &self,
        mut xs: Tensor,
        encoder_output: &Tensor,
        src_mask: bool,
        tgt_mask: bool,
    ) -> Result<Tensor> {
        xs = self.residual_connections[0].forward(
            &xs,
            None,
            src_mask,
            residual_connections::SubLayer::Attention(&self.attention),
        )?;

        xs = self.residual_connections[1].forward(
            &xs,
            Some(encoder_output),
            tgt_mask,
            residual_connections::SubLayer::Attention(&self.cross_attention),
        )?;

        xs = self.residual_connections[2].forward(
            &xs,
            None,
            false,
            residual_connections::SubLayer::FeedForward(&self.feed_forward),
        )?;

        Ok(xs)
    }
}
