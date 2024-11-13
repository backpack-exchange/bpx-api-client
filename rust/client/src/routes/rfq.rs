use bpx_api_types::rfq::{Quote, QuotePayload, RequestForQuote, RequestForQuotePayload};

use crate::error::Result;
use crate::BpxClient;

#[doc(hidden)]
pub const API_RFQ: &str = "/api/v1/rfq";
#[doc(hidden)]
pub const API_RFQ_QUOTE: &str = "/api/v1/rfq/quote";

impl BpxClient {
    pub async fn submit_rfq(&self, payload: RequestForQuotePayload) -> Result<RequestForQuote> {
        let endpoint = format!("{}{}", self.base_url, API_RFQ);
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn submit_quote(&self, payload: QuotePayload) -> Result<Quote> {
        let endpoint = format!("{}{}", self.base_url, API_RFQ_QUOTE);
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }
}