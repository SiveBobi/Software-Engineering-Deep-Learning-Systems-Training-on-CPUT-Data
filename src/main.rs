mod data;
mod model;
mod train;
mod inference;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run -- ask <question>");
        return;
    }

    let question = &args[2];

    // Load model (dummy)
    let model = model::TransformerQA::<burn::tensor::backend::WgpuBackend>::new_dummy();

    let answer = inference::answer_question(&model, question);
    println!("Answer: {}", answer);
}
