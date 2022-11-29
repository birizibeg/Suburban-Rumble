extern crate serde_json;

use std::str;
use serde_json::{Value, from_str};


const AFFIN:&'static [u8; 33051] = include_bytes!("./AFINN-111.json");

pub struct SentimentScore {
        pub positive_score: f64,
        pub negative_score: f64,
        pub net_score: f64,
        pub total_words: f64,
        pub positive_matched_words: i32,
        pub negative_matched_words: i32,

}

pub fn generate_affin_scores(words: &Vec<String>) -> SentimentScore {
    let mut positive_score = 0 as f64;
    let mut positive_words = 0;
    let mut negative_score = 0 as f64;
    let mut negative_words = 0;

    let affin_values = fetch_affin_vals();

    for w in words.to_vec() {
        if let Value::Number(ref val) = affin_values[w] {
            let affin_val = val.as_f64().unwrap();
            if affin_val > 0 as f64 {
                positive_score += affin_val;
                positive_words+=1; 
            } else if affin_val < 0 as f64{
                negative_score += affin_val;
                negative_words+=1;
            }
        }
    }

    SentimentScore {
        positive_score: positive_score,
        negative_score: negative_score,
        net_score: positive_score + negative_score,
        total_words: words.len() as f64,
        positive_matched_words: positive_words,
        negative_matched_words: negative_words
    }
}

pub fn fetch_affin_vals() -> Value{
    let affins: Value = {
            let json = str::from_utf8(AFFIN).unwrap();
            from_str(json).unwrap()
        };
        return affins;
    }
    
