# Word Document Question Answering System

## Overview

This project implements a modular transformer-based Question Answering (QA) system using the Burn deep learning framework in Rust.

The system processes Word documents (.docx), extracts text, tokenizes the content, and performs span-based question answering using a transformer encoder architecture.

---

## System Architecture

### 1. Data Pipeline
- Document loading
- Text extraction
- Tokenization
- Dataset structuring
- Sample generation

### 2. Model Architecture
- Transformer encoder module
- Hidden feature representation
- QA output head (start and end logits)
- Backend-generic implementation

### 3. Training Pipeline
- Forward propagation
- Logit computation
- Modular training structure
- Designed for optimizer integration

### 4. Inference System
- Model loading
- Input processing
- Prediction generation

---

## Project Structure

src/
- data/ → Data pipeline
- model/ → Transformer architecture
- training/ → Training logic
- inference/ → Inference logic

---

## Backend

Burn (NdArray CPU backend)

---

## Running the Project

cargo run

---

## Design Principles

- Modular design
- Separation of concerns
- Backend abstraction
- Clean architecture
- Extendable transformer framework
