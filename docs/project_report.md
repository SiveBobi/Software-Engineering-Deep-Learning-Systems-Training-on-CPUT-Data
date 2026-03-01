# Word-Doc Q&A System using Rust & Burn

## 1. Introduction (10 Marks)

### Problem Statement
Handling and querying institutional calendars manually is time-consuming and error-prone. With multiple events across 2024–2026, such as graduations, committee meetings, and national holidays, staff and students often struggle to find accurate information quickly.

### Motivation
Automating the retrieval of calendar information allows:
- Faster access to event dates
- Accurate counting of meetings
- Efficient planning and scheduling

### Approach Overview
We designed a transformer-based Q&A system in Rust using the Burn deep learning framework. The system:
- Loads Word documents (.docx) containing calendar data
- Tokenizes and processes the text
- Trains a transformer model to answer natural language questions
- Supports a command-line interface for querying

### Key Design Decisions
- Transformer-based architecture for NLP understanding
- Preprocessing calendars to generate Q&A pairs
- Burn framework for Rust-native deep learning
- Command-line interface for lightweight deployment

---

## 2. Implementation (35 Marks)

### 2.1 Architecture Details (20 Marks)
The system implements a **6-layer Transformer encoder** with:
- **Token embeddings**: maps tokens into vector space
- **Positional embeddings**: preserves order of tokens
- **Multi-layer transformer encoder**: self-attention + feedforward
- **Output projection layer**: predicts answer tokens
- **Generic backend**: supports CPU/GPU via Burn’s `Backend` trait

**Model Diagram:**
