use docx_rs::DocxFile;
use tokenizers::{Tokenizer, Encoding};
use std::fs;

// Load a Word document and extract plain text
pub fn load_docx(file_path: &str) -> String {
    let content = fs::read(file_path).expect("File not found");
    let docx = DocxFile::from_reader(&content[..]).unwrap();
    docx.paragraphs()
        .map(|p| p.text().unwrap_or_default())
        .collect::<Vec<String>>()
        .join("\n")
}

// Tokenize text using HuggingFace tokenizer
pub fn tokenize_text(tokenizer: &Tokenizer, text: &str) -> Encoding {
    tokenizer.encode(text, true).unwrap()
}

// Generate Q&A pairs from calendar text
pub fn generate_qa_pairs(text: &str) -> Vec<(String, String)> {
    let mut qa_pairs = Vec::new();

    for line in text.lines() {
        if line.contains("GRADUATION") {
            // Example: extract month and date for graduation
            qa_pairs.push((
                format!("When is the graduation ceremony?"),
                line.to_string(),
            ));
        } else if line.contains("HDC") || line.contains("Higher Degrees Committee") {
            // Count meetings
            qa_pairs.push((
                format!("How many HDC meetings are held?"),
                "1".to_string(), // for simplicity; will count in training preprocessing
            ));
        }
    }

    qa_pairs
}
