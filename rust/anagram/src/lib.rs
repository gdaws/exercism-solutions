use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let (word_lc, word_lcv) = sort_word(word);

    possible_anagrams
        .iter()
        .filter(|x| {
            let (x_lc, x_lcv) = sort_word(x);
            return x_lc != word_lc && x_lcv == word_lcv;
        })
        .cloned()
        .collect()
}

fn sort_word(word: &str) -> (String, Vec<char>) {
    let lc = String::from(word).to_lowercase();
    let mut v: Vec<char> = lc.chars().collect();
    v.sort_unstable();
    (lc, v)
}
