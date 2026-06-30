mod common;

use bpx_api_client::{
    BpxClient,
    types::{fill::Fill, fill::FillsHistoryParams, history::SortDirection, order::Side},
};
use rust_decimal_macros::dec;
use std::collections::BTreeMap;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn get_historical_fills_omits_none_query_params() {
    let mock_server = MockServer::start().await;

    let mock_fills = vec![Fill {
        trade_id: None,
        client_id: None,
        order_id: "1".to_string(),
        symbol: "SOL_USDC".to_string(),
        fee_symbol: "USDC".to_string(),
        price: dec!(1),
        quantity: dec!(100),
        fee: dec!(0),
        side: Side::Ask,
        timestamp: "1970-01-01T00:00:01".to_string(),
        is_maker: false,
        system_order_type: None,
    }];

    Mock::given(method("GET"))
        .and(path("/wapi/v1/history/fills"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            serde_json::to_string(&mock_fills).expect("mock fills should serialize"),
            "application/json",
        ))
        .mount(&mock_server)
        .await;

    let client = BpxClient::builder()
        .base_url(mock_server.uri())
        .secret(&common::test_secret())
        .build()
        .expect("client should build");

    let params = FillsHistoryParams::default()
        .with_from(1000)
        .with_to(2000)
        .with_limit(1000)
        .with_offset(0)
        .with_sort_direction(SortDirection::Asc);

    let fills = client
        .get_historical_fills(params)
        .await
        .expect("request should succeed");

    assert_eq!(fills, mock_fills);

    let requests = mock_server
        .received_requests()
        .await
        .expect("wiremock should record requests");
    assert_eq!(requests.len(), 1);

    let request = &requests[0];
    assert!(request.headers.contains_key("x-signature"));
    assert!(request.headers.contains_key("x-timestamp"));
    assert!(request.headers.contains_key("x-window"));
    assert!(request.headers.contains_key("x-api-key"));

    let query = request
        .url
        .query()
        .expect("fills history request should include query parameters");

    for segment in query.split('&') {
        assert!(
            segment.contains('='),
            "bare query key emitted for unset option: {segment:?} in {query:?}"
        );
    }

    let pairs: BTreeMap<_, _> = query
        .split('&')
        .map(|segment| segment.split_once('=').expect("bare query key"))
        .collect();
    assert_eq!(
        pairs,
        BTreeMap::from([
            ("from", "1000"),
            ("limit", "1000"),
            ("offset", "0"),
            ("sort_direction", "Asc"),
            ("to", "2000"),
        ])
    );
}
