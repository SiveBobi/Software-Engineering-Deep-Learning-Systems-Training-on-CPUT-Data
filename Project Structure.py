word-doc-qa/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI
│   ├── data.rs          # Load & tokenize calendars
│   ├── model.rs         # Transformer QA model
│   ├── train.rs         # Training loop
│   ├── inference.rs     # Question answering
│   └── utils.rs         # Helpers
├── data/
│   ├── calendar_2024.docx
│   ├── calendar_2025.docx
│   └── calendar_2026.docx
├── checkpoints/         # Saved model checkpoints
└── docs/
    └── project_report.md
