use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::{error::Error, io::Write};

pub fn vectorize_obsidian_notes(contents: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("Vectorizing Obsidian notes...");

    // initialize the model. This function is not used frequently. It is fine to call it multiple times.
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()?;

    let sentences = contents;

    let output = model.encode(&sentences)?;
    println!(
        "Vectorization is complete. Vectorized {} documents",
        output.len()
    );

    println!("Embedding size: {}", output[0].len());

    // Create a root file named embeddings.txt
    let mut file = std::fs::File::create("./embeddings_test.txt")?;

    for i in 0..output.len() {
        let mut line = String::new();
        for j in 0..output[i].len() {
            line.push_str(&output[i][j].to_string());
            line.push_str(" ");
        }
        line.push_str("\n");
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}
