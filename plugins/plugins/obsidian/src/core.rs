use crate::aggregator;
use crate::vectorizer::vectorize_obsidian_notes;

pub fn obsidian_demo() {
    let mut aggregator = aggregator::Aggregator::new();
    aggregator.aggregate();

    let data: Vec<Vec<String>> = *aggregator.get_aggregator_data();

    // Prepare the contents in a unique vector
    let mut contents: Vec<String> = Vec::new();
    for i in 0..data.len() {
        contents.push(data[i][1].clone());
    }

    // vectorize the contents
    let _ = vectorize_obsidian_notes(contents);
}
