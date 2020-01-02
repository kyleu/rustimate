use rustimate_core::member::Member;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) struct ResultSummary {
  valid_votes: Vec<(Uuid, String, Option<f64>)>,
  invalid_votes: Vec<(Uuid, Option<String>)>,
  mean: f64,
  median: f64,
  mode: f64
}

impl ResultSummary {
  pub(crate) fn new(members: &[&Member], votes: &[(Uuid, String)]) -> Self {
    let mut valid_votes = Vec::new();
    let mut invalid_votes = Vec::new();

    for m in members {
      let hit = votes
        .iter()
        .find_map(|v| if &v.0 == m.user_id() { Some(v.1.clone()) } else { None });
      match hit {
        Some(h) => {
          let i = match h.parse::<f64>() {
            Ok(n) => Some(n),
            Err(_) => None
          };
          valid_votes.push((*m.user_id(), h, i))
        }
        None => invalid_votes.push((*m.user_id(), None))
      }
    }

    let mut numbers: Vec<f64> = valid_votes.iter().flat_map(|x| x.2).collect();
    numbers.sort_by(|a, b| a.partial_cmp(b).expect("Uncomparable?"));
    let sum = numbers.iter().sum::<f64>();

    let mean = if numbers.is_empty() { 0.0 } else { sum / (numbers.len() as f64) };
    let median = if numbers.is_empty() { 0.0 } else { numbers[numbers.len() / 2] };
    let mode = if numbers.is_empty() {
      0.0
    } else {
      let mut occurrences = HashMap::new();

      for value in numbers {
        *occurrences.entry(value.to_string()).or_insert(0) += 1;
      }

      occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val.parse::<f64>().expect("Did it change somehow?"))
        .expect("Mode attempted with zero numbers")
    };

    Self {
      valid_votes,
      invalid_votes,
      mean,
      median,
      mode
    }
  }

  pub(crate) const fn valid_votes(&self) -> &Vec<(Uuid, String, Option<f64>)> {
    &self.valid_votes
  }

  pub(crate) const fn invalid_votes(&self) -> &Vec<(Uuid, Option<String>)> {
    &self.invalid_votes
  }

  pub(crate) const fn mean(&self) -> f64 {
    self.mean
  }

  pub(crate) const fn median(&self) -> f64 {
    self.median
  }

  pub(crate) const fn mode(&self) -> f64 {
    self.mode
  }
}
