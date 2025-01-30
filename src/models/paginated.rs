use crate::models::realestate::RealEstate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub from: u32,
    pub max_from: u32,
    pub results: Vec<T>,
    pub size: u32,
    pub total: u32,
}

pub fn parse_search_result(str: &str) -> Paginated<RealEstate> {
    serde_json::from_str(str).unwrap()
}

#[cfg(test)]
mod test {
    use crate::models::paginated::parse_search_result;
    use std::fs;

    #[test]
    pub fn parse_result_2() {
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let paginated_result = parse_search_result(&file);

        assert!(paginated_result.total > 0)
    }
}
