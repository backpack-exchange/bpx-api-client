use bpx_api_types::order::{
    CancelOpenOrdersPayload, CancelOrderPayload, ExecuteOrderPayload, Order,
};

use crate::BpxClient;
use crate::error::{Error, Result};

#[doc(hidden)]
pub const API_ORDER: &str = "/api/v1/order";
#[doc(hidden)]
pub const API_ORDERS: &str = "/api/v1/orders";

impl BpxClient {
    /// Fetches a specific open order by symbol and either order ID or client ID.
    pub async fn get_open_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        client_id: Option<u32>,
    ) -> Result<Order> {
        let mut url = self.base_url.join(API_ORDER)?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", symbol);
            if let Some(order_id) = order_id {
                query.append_pair("orderId", order_id);
            } else {
                query.append_pair(
                    "clientId",
                    &client_id
                        .ok_or_else(|| {
                            Error::InvalidRequest("either order_id or client_id is required".into())
                        })?
                        .to_string(),
                );
            }
        }
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Executes a new order with the given payload.
    pub async fn execute_order(&self, payload: ExecuteOrderPayload) -> Result<Order> {
        let endpoint = self.base_url.join(API_ORDER)?;
        let res = self.post(endpoint, payload).await?;
        Self::json_with_context(res).await
    }

    /// Cancels a specific order by symbol and either order ID or client ID.
    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        client_id: Option<u32>,
    ) -> Result<Order> {
        let url = self.base_url.join(API_ORDER)?;
        let payload = CancelOrderPayload {
            symbol: symbol.to_string(),
            order_id: order_id.map(|s| s.to_string()),
            client_id,
        };

        let res = self.delete(url, payload).await?;
        Self::json_with_context(res).await
    }

    /// Retrieves all open orders, optionally filtered by symbol.
    pub async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let mut url = self.base_url.join(API_ORDERS)?;
        if let Some(s) = symbol {
            url.query_pairs_mut().append_pair("symbol", s);
        }
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Cancels all open orders matching the specified payload.
    pub async fn cancel_open_orders(&self, payload: CancelOpenOrdersPayload) -> Result<Vec<Order>> {
        let url = self.base_url.join(API_ORDERS)?;
        let res = self.delete(url, payload).await?;
        Self::json_with_context(res).await
    }
}
