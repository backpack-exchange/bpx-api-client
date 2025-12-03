use bpx_api_types::rfq::{
    Quote, QuoteAcceptPayload, QuotePayload, RequestForQuote, RequestForQuoteCancelPayload,
    RequestForQuotePayload, RequestForQuoteRefreshPayload,
};

#[cfg(feature = "ws")]
use bpx_api_types::rfq::RequestForQuoteUpdate;
#[cfg(feature = "ws")]
use tokio::sync::mpsc::Sender;

use crate::BpxClient;
use crate::error::Result;

#[doc(hidden)]
pub const API_RFQ: &str = "/api/v1/rfq";
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
    pub async fn submit_rfq(&self, payload: RequestForQuotePayload) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ)?;
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn cancel_rfq(
        &self,
        payload: RequestForQuoteCancelPayload,
    ) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ_CANCEL)?;
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn refresh_rfq(
        &self,
        payload: RequestForQuoteRefreshPayload,
    ) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ_REFRESH)?;
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn accept_quote(&self, payload: QuoteAcceptPayload) -> Result<RequestForQuote> {
        let endpoint = self.base_url.join(API_RFQ_ACCEPT)?;
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn submit_quote(&self, payload: QuotePayload) -> Result<Quote> {
        let endpoint = self.base_url.join(API_RFQ_QUOTE)?;
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    #[cfg(feature = "ws")]
    pub async fn subscribe_to_rfqs(&self, tx: Sender<RequestForQuoteUpdate>) -> Result<()> {
        self.subscribe(API_RFQ_STREAM, tx).await
    }
}
