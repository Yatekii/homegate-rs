use std::borrow::Cow;

use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};

use crate::api::request::post_url;
use crate::api::BACKEND_URL;
use crate::models::listing::Category;
use crate::models::paginated::{parse_search_result, Paginated};
use crate::models::realestate::{OfferType, RealEstate};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct FromTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<u32>,
    pub geo_tags: Vec<Cow<'static, str>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub categories: Vec<Category>,
    pub exclude_categories: Vec<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub living_space: Option<FromTo>,
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_rent: Option<FromTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<FromTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_rooms: Option<FromTo>,
    pub offer_type: OfferType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GeoCoordsTemplate {
    pub latitude: bool,
    pub longitude: bool,
}

impl Default for GeoCoordsTemplate {
    fn default() -> Self {
        Self {
            latitude: true,
            longitude: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressTemplate {
    pub country: bool,
    pub geo_coordinates: GeoCoordsTemplate,
    pub locality: bool,
    pub post_office_box_number: bool,
    pub postal_code: bool,
    pub region: bool,
    pub street: bool,
    pub street_addition: bool,
}

impl Default for AddressTemplate {
    fn default() -> Self {
        Self {
            country: true,
            geo_coordinates: Default::default(),
            locality: true,
            post_office_box_number: true,
            postal_code: true,
            region: true,
            street: true,
            street_addition: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicsTemplate {
    pub living_space: bool,
    pub lot_size: bool,
    pub number_of_rooms: bool,
    pub single_floor_space: bool,
    pub total_floor_space: bool,
}

impl Default for CharacteristicsTemplate {
    fn default() -> Self {
        Self {
            living_space: true,
            lot_size: true,
            number_of_rooms: true,
            single_floor_space: true,
            total_floor_space: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListerTemplate {
    pub logo_url: bool,
    pub phone: bool,
}

impl Default for ListerTemplate {
    fn default() -> Self {
        Self {
            logo_url: false,
            phone: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTextTemplate {
    pub title: bool,
}

impl Default for LocaleTextTemplate {
    fn default() -> Self {
        Self { title: true }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocaleUrlsTemplate {
    #[serde(rename = "type")]
    pub t: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTemplate {
    pub attachments: bool,
    pub text: LocaleTextTemplate,
    pub urls: LocaleUrlsTemplate,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationTemplate {
    pub de: LocaleTemplate,
    pub en: LocaleTemplate,
    pub fr: LocaleTemplate,
    pub it: LocaleTemplate,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListingTemplate {
    pub address: AddressTemplate,
    pub categories: bool,
    pub characteristics: CharacteristicsTemplate,
    pub id: bool,
    pub lister: ListerTemplate,
    pub localization: LocalizationTemplate,
    pub offer_type: bool,
    pub prices: bool,
}

impl Default for ListingTemplate {
    fn default() -> Self {
        Self {
            address: Default::default(),
            categories: true,
            characteristics: Default::default(),
            id: true,
            lister: Default::default(),
            localization: Default::default(),
            offer_type: true,
            prices: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResultTemplate {
    pub id: bool,
    pub lister_branding: bool,
    pub listing: ListingTemplate,
    pub listing_type: bool,
    pub remote_viewing: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub from: i32,
    pub query: Query,
    pub result_template: ResultTemplate,
    pub size: i32,
    pub sort_by: SortBy,
    pub sort_direction: SortDirection,
    pub track_total_hits: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    #[default]
    Desc,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum SortBy {
    #[default]
    ListingType,
}

pub async fn search(params: &SearchRequest) -> Result<Paginated<RealEstate>, reqwest::Error> {
    let url: Url = Url::parse(&format!("{}{}", BACKEND_URL, "/search/listings")).unwrap();

    let search_request_json = serde_json::to_string(&params).unwrap();

    let resp: Response = post_url(url, &search_request_json).await?;
    let resp_text = resp.text().await?;
    let r: Paginated<RealEstate> = parse_search_result(&resp_text);
    Ok(r)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::api::search::{search, Location, SearchRequest};

    const ZURICH_LATLNG: (f64, f64) = (47.36667, 8.55);

    #[tokio::test]
    pub async fn search_apartment() {
        // let paginated_result = search(&Location {
        //     latitude: ZURICH_LATLNG.0 as f32,
        //     longitude: ZURICH_LATLNG.1 as f32,
        //     radius: 1000,
        // })
        // .await;
        // assert!(paginated_result.is_ok());

        // let pr = paginated_result.unwrap();
        // println!("{:?}", pr);
    }

    #[test]
    pub fn create_json() {
        let req = SearchRequest::default();
        let _v = serde_json::to_string(&req).unwrap();
        let f_json = fs::read_to_string("./resources/test/request-1.json").unwrap();

        let decoded_json: SearchRequest = serde_json::from_str(f_json.as_str()).unwrap();
        assert_eq!(decoded_json, req);
    }
}
