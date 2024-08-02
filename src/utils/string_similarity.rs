use std::collections::HashSet;

fn generate_trigrams(s: &str) -> HashSet<String> {
    let mut trigrams = HashSet::new();
    let s = s.to_lowercase(); // Normalize to lowercase
    let chars: Vec<char> = s.chars().collect();

    if chars.len() < 3 {
        return trigrams;
    }

    for i in 0..=chars.len() - 3 {
        let trigram: String = chars[i..i + 3].iter().collect();
        trigrams.insert(trigram);
    }

    trigrams
}

pub fn trigram_similarity(s1: &str, s2: &str) -> f64 {
    let trigrams1 = generate_trigrams(s1);
    let trigrams2 = generate_trigrams(s2);

    if trigrams1.is_empty() || trigrams2.is_empty() {
        return 0.0;
    }

    let intersection: HashSet<_> = trigrams1.intersection(&trigrams2).collect();
    let union: HashSet<_> = trigrams1.union(&trigrams2).collect();

    intersection.len() as f64 / union.len() as f64
}
