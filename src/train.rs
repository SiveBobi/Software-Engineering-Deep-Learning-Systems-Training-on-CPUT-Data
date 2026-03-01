use crate::model::TransformerQA;
use burn::optim::Adam;
use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

pub fn train<B: Backend>(
    model: &mut TransformerQA<B>,
    dataset: &Vec<(Tensor<B, 2>, Tensor<B, 2>)>,
    epochs: usize,
    learning_rate: f32,
) {
    let mut optimizer = Adam::new(learning_rate);

    for epoch in 0..epochs {
        for (question, answer) in dataset.iter() {
            let logits = model.forward(question.clone());
            let loss = logits.cross_entropy(answer.clone());
            optimizer.backward_step(&mut model, &loss);
        }
        println!("Epoch {} completed", epoch);
        burn::train::checkpoint::save(model, &format!("checkpoints/epoch_{}.ckpt", epoch))
            .unwrap();
    }
}
