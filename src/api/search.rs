use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};

use crate::api::request::post_url;
use crate::api::BACKEND_URL;
use crate::models::listing::Category;
use crate::models::paginated::{parse_search_result, Paginated};
use crate::models::realestate::{OfferType, RealEstate};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FromTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
    pub radius: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub categories: Vec<String>,
    pub exclude_categories: Vec<String>,
    pub living_space: FromTo,
    pub location: Location,
    pub monthly_rent: FromTo,
    pub number_of_rooms: FromTo,
    pub offer_type: OfferType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GeoCoordsTemplate {
    pub latitude: bool,
    pub longitude: bool,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicsTemplate {
    pub living_space: bool,
    pub lot_size: bool,
    pub number_of_rooms: bool,
    pub single_floor_space: bool,
    pub total_floor_space: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListerTemplate {
    pub logo_url: bool,
    pub phone: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTextTemplate {
    pub title: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleUrlsTemplate {
    #[serde(rename = "type")]
    pub t: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTemplate {
    pub attachments: bool,
    pub text: LocaleTextTemplate,
    pub urls: LocaleUrlsTemplate,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultTemplate {
    pub id: bool,
    pub lister_branding: bool,
    pub listing: ListingTemplate,
    pub listing_type: bool,
    pub remote_viewing: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub from: i32,
    pub query: Query,
    pub result_template: ResultTemplate,
    pub size: i32,
    pub sort_by: String,
    pub sort_direction: String,
    pub track_total_hits: bool,
}

const LT: LocaleTemplate = LocaleTemplate {
    urls: LocaleUrlsTemplate { t: true },
    attachments: true,
    text: LocaleTextTemplate { title: true },
};

pub fn default_search<'a>() -> SearchRequest {
    SearchRequest {
        from: 0,
        query: Query {
            categories: Vec::from(vec![
                Category::Apartment,
                Category::Maisonette,
                Category::Duplex,
                Category::AtticFlat,
                Category::RoofFlat,
                Category::Studio,
                Category::SingleRoom,
                Category::TerraceFlat,
                Category::BachelorFlat,
                Category::Loft,
                Category::Attic,
                Category::RowHouse,
                Category::BifamiliarHouse,
                Category::TerraceHouse,
                Category::Villa,
                Category::FarmHouse,
                Category::CaveHouse,
                Category::Castle,
                Category::GrannyFlat,
                Category::Chalet,
                Category::Rustico,
                Category::SingleHouse,
                Category::HobbyRoom,
                Category::CellarCompartment,
                Category::AtticCompartment,
            ])
            .iter()
            .map(|c| c.to_string())
            .collect(),
            exclude_categories: Vec::from(vec![Category::FurnishedFlat])
                .iter()
                .map(|c| c.to_string())
                .collect(),
            living_space: FromTo {
                from: Some(60),
                to: None,
            },
            location: Location {
                latitude: 47.35985528332324,
                longitude: 8.541818987578152,
                radius: 622,
            },
            monthly_rent: FromTo {
                from: Some(500),
                to: None,
            },
            number_of_rooms: FromTo {
                from: Some(2),
                to: None,
            },
            offer_type: OfferType::RENT,
        },
        result_template: ResultTemplate {
            id: true,
            lister_branding: true,
            listing: ListingTemplate {
                address: AddressTemplate {
                    country: true,
                    geo_coordinates: GeoCoordsTemplate {
                        latitude: true,
                        longitude: true,
                    },
                    locality: true,
                    post_office_box_number: true,
                    postal_code: true,
                    region: true,
                    street: true,
                    street_addition: true,
                },
                categories: true,
                characteristics: CharacteristicsTemplate {
                    living_space: true,
                    lot_size: true,
                    number_of_rooms: true,
                    single_floor_space: true,
                    total_floor_space: true,
                },
                id: true,
                lister: ListerTemplate {
                    logo_url: true,
                    phone: true,
                },
                localization: LocalizationTemplate {
                    de: LT.clone(),
                    en: LT.clone(),
                    fr: LT.clone(),
                    it: LT.clone(),
                    primary: true,
                },
                offer_type: true,
                prices: true,
            },
            listing_type: true,
            remote_viewing: true,
        },
        size: 20,
        sort_by: String::from("listingType"),
        sort_direction: String::from("desc"),
        track_total_hits: true,
    }
}

pub async fn search(location: &Location) -> Result<Paginated<RealEstate>, reqwest::Error> {
    let url: Url = Url::parse(&format!("{}{}", BACKEND_URL, "/search/listings")).unwrap();

    let mut search_request = default_search();
    search_request.query.location = location.clone();

    let search_request_json = serde_json::to_string(&search_request).unwrap();

    let resp: Response = post_url(url, &search_request_json).await?;
    let resp_text = resp.text().await?;
    let r: Paginated<RealEstate> = parse_search_result(&resp_text);
    Ok(r)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::api::search::{default_search, search, Location, SearchRequest};

    const ZURICH_LATLNG: (f64, f64) = (47.36667, 8.55);

    #[tokio::test]
    pub async fn search_apartment() {
        let paginated_result = search(&Location {
            latitude: ZURICH_LATLNG.0 as f32,
            longitude: ZURICH_LATLNG.1 as f32,
            radius: 1000,
        })
        .await;
        assert!(paginated_result.is_ok());

        let pr = paginated_result.unwrap();
        println!("{:?}", pr);
    }

    #[test]
    pub fn create_json() {
        let req = default_search();
        let _v = serde_json::to_string(&req).unwrap();
        let f_json = fs::read_to_string("./resources/test/request-1.json").unwrap();

        let decoded_json: SearchRequest = serde_json::from_str(f_json.as_str()).unwrap();
        assert_eq!(decoded_json, req);
    }
}
