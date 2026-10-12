use bpx_api_types::rfq::{
    OpenRfqsQuery, Quote, QuoteAcceptPayload, QuotePayload, RequestForQuote,
    RequestForQuoteCancelPayload, RequestForQuotePayload, RequestForQuoteRefreshPayload,
    RfqWithQuotes,
};

#[cfg(feature = "ws")]
use bpx_api_types::rfq::RequestForQuoteUpdate;
#[cfg(feature = "ws")]
use tokio::sync::mpsc::Sender;

use crate::BpxClient;
use crate::error::{Error, Result};

#[doc(hidden)]
pub const API_RFQ: &str = "/api/v1/rfq";
#[doc(hidden)]
pub const API_RFQS: &str = "/api/v1/rfqs";
#[doc(hidden)]
pub const API_RFQ_QUOTE: &str = "/api/v1/rfq/quote";
#[doc(hidden)]
pub const API_RFQ_CANCEL: &str = "/api/v1/rfq/cancel";
#[doc(hidden)]
pub const API_RFQ_REFRESH: &str = "/api/v1/rfq/refresh";
#[doc(hidden)]
pub const API_RFQ_ACCEPT: &str = "/api/v1/rfq/accept";

#[cfg(feature = "ws")]
const API_RFQ_STREAM: &str = "account.rfqUpdate";

impl BpxClient {
    /// Fetches the account's open RFQs and their quotes.
    pub async fn get_open_rfqs(&self, params: OpenRfqsQuery) -> Result<Vec<RfqWithQuotes>> {
        let mut url = self.base_url.join(API_RFQS)?;
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| Error::UrlParseError(e.to_string().into_boxed_str()))?;
        if !query_string.is_empty() {
            url.set_query(Some(&query_string));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Submits an RFQ. `order_expiry_time` makes it a resting RFQ.
    pub async fn submit_rfq(&self, payload: RequestForQuotePayload) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ)?;
        let res = self.post(endpoint, payload).await?;
        Self::deserialize_json(res).await
    }

    /// Cancels an RFQ. A bound resting RFQ is only marked (`cancel_requested_at`)
    /// and stays active until the quoter or system settles or cancels it.
    pub async fn cancel_rfq(
        &self,
        payload: RequestForQuoteCancelPayload,
    ) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ_CANCEL)?;
        let res = self.post(endpoint, payload).await?;
        Self::deserialize_json(res).await
    }

    pub async fn refresh_rfq(
        &self,
        payload: RequestForQuoteRefreshPayload,
    ) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ_REFRESH)?;
        let res = self.post(endpoint, payload).await?;
        Self::deserialize_json(res).await
    }

    pub async fn accept_quote(&self, payload: QuoteAcceptPayload) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ_ACCEPT)?;
        let res = self.post(endpoint, payload).await?;
        Self::deserialize_json(res).await
    }

    pub async fn submit_quote(&self, payload: QuotePayload) -> Result<Quote> {
        let endpoint = self.base_url.join(API_RFQ_QUOTE)?;
        let res = self.post(endpoint, payload).await?;
        Self::deserialize_json(res).await
    }

    #[cfg(feature = "ws")]
    pub async fn subscribe_to_rfqs(&self, tx: Sender<RequestForQuoteUpdate>) -> Result<()> {
        self.subscribe(API_RFQ_STREAM, tx).await
    }
}
