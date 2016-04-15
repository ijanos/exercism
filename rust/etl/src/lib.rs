use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter()
         .flat_map(|(&score, values)| {
             values.iter().map(move |value| (value.to_string().to_lowercase(), score))
         })
         .collect()
}
