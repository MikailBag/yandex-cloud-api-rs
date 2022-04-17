const ENDPOINTS_API_ENDPOINT: &str = "https://api.cloud.yandex.net/endpoints";

#[derive(serde_derive::Deserialize)]
struct Endpoint {
    id: String,
    address: String,
}

#[derive(serde_derive::Deserialize)]
struct Endpoints {
    endpoints: Vec<Endpoint>,
}

#[test]
fn check_service_naming() {
    let catalog = crate::catalog::services();
    let endpoints_info: Endpoints = reqwest::blocking::get(ENDPOINTS_API_ENDPOINT)
        .expect("failed to fetch Yandex Cloud endpoints")
        .json()
        .expect("failed to parse API response");
    for svc in catalog {
        let expected = endpoints_info
            .endpoints
            .iter()
            .find(|info| info.id == svc.name())
            .unwrap_or_else(|| panic!("Service {} not found ", svc.name()));
        assert_eq!(expected.address, svc.endpoint().to_string() + ":443");
    }
}
