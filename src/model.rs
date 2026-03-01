use burn::nn::{Module, Linear, LayerNorm, Embedding};
use burn::tensor::backend::Backend;

// Transformer Encoder Layer
pub struct TransformerLayer<B: Backend> {
    pub ln1: LayerNorm<B>,
    pub ln2: LayerNorm<B>,
    pub ff: Linear<B>,
}

impl<B: Backend> TransformerLayer<B> {
    pub fn forward(&self, x: burn::tensor::Tensor<B, 2>) -> burn::tensor::Tensor<B, 2> {
        let h = x.clone();
        let h = self.ln1.forward(h);
        let h = self.ff.forward(h);
        self.ln2.forward(h + x)
    }
}

// Full Transformer QA model
pub struct TransformerQA<B: Backend> {
    pub token_embedding: Embedding<B>,
    pub position_embedding: Embedding<B>,
    pub transformer_layers: Vec<TransformerLayer<B>>,
    pub output_projection: Linear<B>,
}

impl<B: Backend> Module<B> for TransformerQA<B> {
    fn forward(&self, input: burn::tensor::Tensor<B, 2>) -> burn::tensor::Tensor<B, 2> {
        let mut x = self.token_embedding.forward(input.clone());
        x = x + self.position_embedding.forward(input);
        for layer in &self.transformer_layers {
            x = layer.forward(x);
        }
        self.output_projection.forward(x)
    }
}
