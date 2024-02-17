use std::error::Error;

use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use sled;
pub struct PlumVector {
    vector: sled::Db,
}

impl PlumVector {
    pub fn new() -> Self {
        PlumVector {
            vector: sled::open("plum_vector").unwrap(),
        }
    }
}

pub fn embed_sentences(docs: Vec<String>) -> Option<Vec<Vec<f32>>> {
    // Don't use the heap for the model - allocate inside the function
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()
        .unwrap();

    let output = model.encode(&docs).unwrap();
    Some(output)
}
