use reqwest::{Response, Url};

use crate::models::listing::ListingResponse;

use super::{request::get_url, BACKEND_URL};

pub async fn listing(ids: Vec<usize>) -> Result<ListingResponse, reqwest::Error> {
    let id_string = ids.iter().fold(String::new(), |state, id| {
        format!("{}{state}{id}", if state.is_empty() { "" } else { "," })
    });
    let url: Url = Url::parse(&format!(
        "{}{}{}",
        BACKEND_URL, "/listings/listings?ids=", id_string
    ))
    .unwrap();

    let resp: Response = get_url(url).await?;
    let resp_text = resp.text().await?;
    let r = parse_listings_result(&resp_text);
    Ok(r)
}

pub fn parse_listings_result(str: &str) -> ListingResponse {
    serde_json::from_str(str).unwrap()
}

#[cfg(test)]
mod tests {

    use crate::api::listing::listing;

    #[tokio::test]
    pub async fn list_apartment() {
        let paginated_result = listing(vec![4001792114]).await;
        assert!(paginated_result.is_ok());

        let pr = paginated_result.unwrap();
        println!("{:?}", pr);
    }
}
