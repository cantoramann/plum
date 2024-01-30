// mod embeddings;
mod plugins;

fn main() {
    // let mut aggregator = obsidian::aggregator::Aggregator::new();
    // aggregator.aggregate();

    // let data: Vec<Vec<String>> = *aggregator.get_aggregator_data();

    // // Prepare the contents in a unique vector
    // let mut contents: Vec<String> = Vec::new();
    // for i in 0..data.len() {
    //     contents.push(data[i][1].clone());
    // }

    // // vectorize the contents
    // let _ = embeddings::vectorizer::vectorize(contents);

    let coord = plugins::coordinator::PlumCoordinator::new();
    coord.run("test".to_string());
}
