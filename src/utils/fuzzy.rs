use std::char::ToLowercase;
use std::cmp::max;

pub fn scored_fuzzy_search<'a>(pattern: &'a str, st: &'a str) -> (bool, i64) {
    // https://github.com/tajmone/fuzzy-search
    // 1:1 port of python implementation in above project
    // Don't know if this can be improved, but works for now.
    //
    let separator_bonus = 10;
    let adj_bonus = 5;
    let camel_case_bonus = 10;

    let unmatched_penalty = -1;
    let lead_penalty = -3;
    let max_lead_penalty = -9;

    let mut p_idx = 0_usize;
    let (mut s_idx, p_len, s_len) = (0, pattern.len(), st.len());
    let (mut prev_match, mut prev_lower) = (false, false);
    let mut prev_sep = true;

    let mut best_letter: Option<char> = None;
    let mut best_lower: Option<ToLowercase> = None;
    let mut best_letter_idx: Option<usize> = None;
    let mut best_letter_score = 0;
    let mut matched_indices: Vec<usize> = Vec::new();

    let mut score = 0;

    while s_idx < s_len {
        let p_char = pattern.chars().nth(p_idx);
        let p_lower = p_char.map(|val| val.to_lowercase().to_string());
        let s_char = st.chars().nth(s_idx).unwrap();
        let s_lower = s_char.to_lowercase().to_string();
        let s_upper = s_char.to_uppercase().to_string();

        let next_match = p_char.is_some()
            && if let Some(val) = &p_lower {
                val == &s_lower.to_string()
            } else {
                false
            };

        let rematch = if let Some(b_val) = &best_lower {
            if let Some(p_val) = &p_lower {
                p_val == &b_val.to_string()
            } else {
                false
            }
        } else {
            false
        };

        let advanced = next_match && best_letter.is_some();
        let p_repeat = best_letter.is_some()
            && p_char.is_some()
            && best_lower.clone().unwrap().to_string() == p_lower.unwrap();

        if advanced || p_repeat {
            score += best_letter_score;
            matched_indices.push(best_letter_idx.unwrap());
            best_letter = None;
            best_lower = None;
            best_letter_idx = None;
            best_letter_score = 0;
        }

        if next_match || rematch {
            let mut new_score = 0;

            if p_idx == 0 {
                score += max(s_idx as i64 * lead_penalty, max_lead_penalty);
            }

            if prev_match {
                new_score += adj_bonus;
            }

            if prev_sep {
                new_score += separator_bonus;
            }

            if prev_lower && s_char.to_string() == s_upper && s_lower != s_upper {
                new_score += camel_case_bonus;
            }

            if next_match {
                p_idx += 1;
            }

            if new_score > best_letter_score {
                if best_letter.is_some() {
                    score += unmatched_penalty;
                }
                best_letter = Some(s_char);
                best_lower = Some(s_char.to_lowercase());
                best_letter_idx = Some(s_idx);
                best_letter_score = new_score;
            }
            prev_match = true;
        } else {
            score += unmatched_penalty;
            prev_match = false;
        }

        prev_lower = s_char.to_string() == s_lower && s_lower != s_upper;
        prev_sep = "_ ".contains(s_char);

        s_idx += 1;
    }
    if best_letter.is_some() {
        score += best_letter_score;
        matched_indices.push(best_letter_idx.unwrap());
    }

    (p_idx == p_len, score)
}
