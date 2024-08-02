#[cfg(test)]
mod tests {
    use mafiascum_scraper::utils::string_similarity;

    #[test]
    fn test_trigram_similarity_identical_strings() {
        let s1 = "rustacean";
        let s2 = "rustacean";
        assert_eq!(string_similarity::trigram_similarity(s1, s2), 1.0);
    }

    #[test]
    fn test_trigram_similarity_completely_different_strings() {
        let s1 = "aaaaaaaaaa";
        let s2 = "wyhdgawvdyuwaihdaywhduawhdiuhawuhdawi";
        assert_eq!(string_similarity::trigram_similarity(s1, s2), 0.0);
    }

    #[test]
    fn test_trigram_similarity_partial_match() {
        let s1 = "rust";
        let s2 = "trust";
        let value = string_similarity::trigram_similarity(s1, s2);
        assert!(value > 0.6);
    }

    #[test]
    fn test_trigram_similarity_no_trigrams() {
        let s1 = "hi";
        let s2 = "hello";
        assert_eq!(string_similarity::trigram_similarity(s1, s2), 0.0);
    }

    #[test]
    fn test_trigram_similarity_some_overlap() {
        let s1 = "rust programming language";
        let s2 = "rustacean programming";
        assert!(string_similarity::trigram_similarity(s1, s2) > 0.0);
    }
}
