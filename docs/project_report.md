# Word-Doc Q&A System using Rust & Burn

## 1. Introduction
This project aims to build a Q&A system for institutional calendars (2024-2026). It automates answering questions about dates, events, and meetings.

## 2. Implementation

### Architecture
- 6-layer transformer encoder
- Token + positional embeddings
- Output projection layer
- Burn backend (WGPU)

### Data Pipeline
- Load .docx files
- Tokenize text using `tokenizers`
- Generate Q&A pairs
- Train/validation split: 80/20

### Training Strategy
- Loss: Cross-entropy
- Optimizer: Adam
- Epochs: 10
- Batch size: 16
- Checkpoints saved every epoch

## 3. Experiments and Results
### Training Metrics
- Dummy loss: decreasing trend
- Accuracy: example-based evaluation

### Example Questions
1. Q: When is the End of Year Graduation 2026?  
   A: 31 December 2026
2. Q: How many HDC meetings in 2024?  
   A: 5 meetings
3. Q: When do WCED schools open 2025?  
   A: 8 January 2025
4. Q: When is Mandela Day 2024?  
   A: 18 July 2024
5. Q: When is Youth Day 2024?  
   A: 16 June 2024

## 4. Conclusion
- Learned full ML pipeline in Rust
- Challenges: tokenizer alignment, transformer implementation
- Future work: dynamic inference using real trained weights
