use bpx_api_types::rfq::{Quote, QuotePayload, RequestForQuote, RequestForQuotePayload};

#[cfg(feature = "ws")]
use bpx_api_types::rfq::RequestForQuoteUpdate;
#[cfg(feature = "ws")]
use tokio::sync::mpsc::Sender;

use crate::error::Result;
use crate::BpxClient;

#[doc(hidden)]
pub const API_RFQ: &str = "/api/v1/rfq";
#[doc(hidden)]
pub const API_RFQ_QUOTE: &str = "/api/v1/rfq/quote";

#[cfg(feature = "ws")]
const API_RFQ_STREAM: &str = "account.rfqUpdate";

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

    #[cfg(feature = "ws")]
    pub async fn subscribe_to_rfqs(&self, tx: Sender<RequestForQuoteUpdate>) {
        self.subscribe(API_RFQ_STREAM, tx).await;
    }
}
