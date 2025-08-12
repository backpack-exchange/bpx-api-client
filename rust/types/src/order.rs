use std::{fmt, str::FromStr};

use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerBy {
    LastPrice,
    MarkPrice,
    IndexPrice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerQuantity {
    Percent(Decimal),
    Amount(Decimal),
}

impl<'de> Deserialize<'de> for TriggerQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QtyVisitor;

        impl Visitor<'_> for QtyVisitor {
            type Value = TriggerQuantity;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#"a string like "12.5%" or "0.01", or a number"#)
            }

            // ---------- JSON string ----------
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                parse_str(v).map_err(serde::de::Error::custom)
            }

            // ---------- JSON numbers ----------
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Decimal::from_f64(v)
                    .ok_or_else(|| serde::de::Error::custom("not a finite number"))
                    .map(TriggerQuantity::Amount)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TriggerQuantity::Amount(Decimal::from(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TriggerQuantity::Amount(Decimal::from(v)))
            }
        }

        deserializer.deserialize_any(QtyVisitor)
    }
}

impl Serialize for TriggerQuantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            match self {
                Self::Percent(percent) => format!("{percent}%"),
                Self::Amount(amount) => format!("{amount}"),
            }
            .as_str(),
        )
    }
}

fn parse_str(s: &str) -> Result<TriggerQuantity, &'static str> {
    if let Some(num) = s.strip_suffix('%') {
        let d = Decimal::from_str(num.trim()).map_err(|_| "invalid percent value")?;
        Ok(TriggerQuantity::Percent(d))
    } else {
        let d = Decimal::from_str(s.trim()).map_err(|_| "invalid decimal value")?;
        Ok(TriggerQuantity::Amount(d))
    }
}

/// Custom timestamp deserializer that handles both i64 and ISO string formats
mod flexible_timestamp {
    use serde::{de::Visitor, Deserialize, Deserializer};
    use std::fmt;

    struct TimestampVisitor;

    impl Visitor<'_> for TimestampVisitor {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer timestamp or ISO datetime string")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as i64)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            // Try parsing as integer first
            if let Ok(num) = v.parse::<i64>() {
                return Ok(num);
            }

            // Parse ISO datetime string using chrono NaiveDateTime
            // Handle format: "2025-07-24T04:05:48.931" (no timezone)
            let naive_datetime = chrono::NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3f")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S"))
                .map_err(|e| serde::de::Error::custom(format!("Invalid datetime format '{}': {}", v, e)))?;

            // Convert to UTC timestamp in milliseconds
            Ok(naive_datetime.and_utc().timestamp_millis())
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TimestampVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Option<Decimal>,
    pub executed_quantity: Decimal,
    pub quote_quantity: Option<Decimal>,
    pub executed_quote_quantity: Decimal,
    pub stop_loss_trigger_price: Option<Decimal>,
    pub stop_loss_limit_price: Option<Decimal>,
    pub stop_loss_trigger_by: Option<Decimal>,
    pub take_profit_trigger_price: Option<Decimal>,
    pub take_profit_limit_price: Option<Decimal>,
    pub take_profit_trigger_by: Option<Decimal>,
    pub trigger_by: Option<TriggerBy>,
    pub trigger_price: Option<Decimal>,
    pub trigger_quantity: Option<TriggerQuantity>,
    pub triggered_at: Option<i64>,
    pub time_in_force: TimeInForce,
    pub related_order_id: Option<String>,
    pub self_trade_prevention: SelfTradePrevention,
    pub reduce_only: Option<bool>,
    pub status: OrderStatus,
    #[serde(deserialize_with = "flexible_timestamp::deserialize")]
    pub created_at: i64,
    // Optional fields for order history compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_order_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub executed_quantity: Decimal,
    pub executed_quote_quantity: Decimal,
    pub stop_loss_trigger_price: Option<Decimal>,
    pub stop_loss_limit_price: Option<Decimal>,
    pub stop_loss_trigger_by: Option<Decimal>,
    pub take_profit_trigger_price: Option<Decimal>,
    pub take_profit_limit_price: Option<Decimal>,
    pub take_profit_trigger_by: Option<Decimal>,
    pub price: Decimal,
    pub trigger_by: Option<TriggerBy>,
    pub trigger_price: Option<Decimal>,
    pub trigger_quantity: Option<TriggerQuantity>,
    pub triggered_at: Option<i64>,
    pub time_in_force: TimeInForce,
    pub related_order_id: Option<String>,
    pub self_trade_prevention: SelfTradePrevention,
    pub post_only: bool,
    pub reduce_only: Option<bool>,
    pub status: OrderStatus,
    #[serde(deserialize_with = "flexible_timestamp::deserialize")]
    pub created_at: i64,
    // Optional fields for order history compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_order_type: Option<String>,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderType {
    #[default]
    #[serde(rename(deserialize = "LIMIT"))]
    Limit,
    #[serde(rename(deserialize = "MARKET"))]
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "orderType")]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SelfTradePrevention {
    #[default]
    RejectTaker,
    RejectMaker,
    RejectBoth,
    Allow,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    Cancelled,
    Expired,
    Filled,
    #[default]
    New,
    PartiallyFilled,
    Triggered,
    TriggerPending,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    #[default]
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOrderPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_redeem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow_repay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention: Option<SelfTradePrevention>,
    pub side: Side,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_quantity: Option<TriggerQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderPayload {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOpenOrdersPayload {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderUpdateType {
    OrderAccepted,
    OrderCancelled,
    OrderExpired,
    OrderFill,
    OrderModified,
    TriggerPlaced,
    TriggerFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: OrderUpdateType,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Client order id
    #[serde(rename = "c")]
    pub client_order_id: Option<u64>,

    /// Side
    #[serde(rename = "S")]
    pub side: Side,

    /// Order type
    #[serde(rename = "o")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,

    /// Quantity
    #[serde(rename = "q")]
    pub quantity: Decimal,

    /// Quantity in quote
    #[serde(rename = "Q")]
    pub quantity_in_quote: Option<Decimal>,

    /// price
    #[serde(rename = "p")]
    pub price: Option<Decimal>,

    /// trigger price
    #[serde(rename = "P")]
    pub trigger_price: Option<Decimal>,

    /// trigger by
    #[serde(rename = "B")]
    pub trigger_by: Option<TriggerBy>,

    /// Take profit trigger price
    #[serde(rename = "a")]
    pub take_profit_trigger_price: Option<Decimal>,

    /// Stop loss trigger price
    #[serde(rename = "b")]
    pub stop_loss_trigger_price: Option<Decimal>,

    /// Take profit trigger by
    #[serde(rename = "d")]
    pub take_profit_trigger_by: Option<TriggerBy>,

    /// Stop loss trigger by
    #[serde(rename = "g")]
    pub stop_loss_trigger_by: Option<TriggerBy>,

    /// Trigger quantity
    #[serde(rename = "Y")]
    pub trigger_quantity: Option<Decimal>,

    /// Order State
    #[serde(rename = "X")]
    pub order_status: OrderStatus,

    /// Order expiry reason
    #[serde(rename = "R")]
    pub order_expiry_reason: Option<String>,

    /// Order ID
    #[serde(rename = "i")]
    pub order_id: String,

    /// Trade ID
    #[serde(rename = "t")]
    pub trade_id: Option<u64>,

    /// Fill quantity
    #[serde(rename = "l")]
    pub fill_quantity: Option<Decimal>,

    /// Executed quantity
    #[serde(rename = "z")]
    pub executed_quantity: Decimal,

    /// Executed quantity in quote
    #[serde(rename = "Z")]
    pub executed_quantity_in_quote: Decimal,

    /// Fill price
    #[serde(rename = "L")]
    pub fill_price: Option<Decimal>,

    /// Fill price
    #[serde(rename = "m")]
    pub was_maker: Option<bool>,

    /// Fee
    #[serde(rename = "n")]
    pub fee: Option<Decimal>,

    /// Fee symbol
    #[serde(rename = "N")]
    pub fee_symbol: Option<String>,

    /// Self trade prevention
    #[serde(rename = "V")]
    pub self_trade_prevention: SelfTradePrevention,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: i64,

    /// Origin of the update
    #[serde(rename = "O")]
    pub origin_of_the_update: String,

    /// Related order ID
    #[serde(rename = "I")]
    pub related_order_id: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use serde_json::json;

    #[test]
    fn both_forms_round_trip() {
        let q: TriggerQuantity = serde_json::from_value(json!("12.5%")).unwrap();
        assert_eq!(q, TriggerQuantity::Percent(dec!(12.5)));

        let q: TriggerQuantity = serde_json::from_value(json!("0.01")).unwrap();
        assert_eq!(q, TriggerQuantity::Amount(dec!(0.01)));
    }

    #[test]
    fn test_trigger_quantity_serialize() {
        let trigger_quantity = TriggerQuantity::Percent(dec!(100));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"100%\"");

        let trigger_quantity = TriggerQuantity::Percent(dec!(75.50));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"75.50%\"");

        let trigger_quantity = TriggerQuantity::Amount(dec!(100));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"100\"");

        let trigger_quantity = TriggerQuantity::Amount(dec!(75.50));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"75.50\"");
    }

    #[test]
    fn test_trigger_by_serialize() {
        let trigger_by_last = TriggerBy::LastPrice;
        let trigger_by_last_str = serde_json::to_string(&trigger_by_last).unwrap();
        assert_eq!(trigger_by_last_str, "\"LastPrice\"");

        let trigger_by_mark = TriggerBy::MarkPrice;
        let trigger_by_mark_str = serde_json::to_string(&trigger_by_mark).unwrap();
        assert_eq!(trigger_by_mark_str, "\"MarkPrice\"");

        let trigger_by_index = TriggerBy::IndexPrice;
        let trigger_by_index_str = serde_json::to_string(&trigger_by_index).unwrap();
        assert_eq!(trigger_by_index_str, "\"IndexPrice\"");
    }

    #[test]
    fn test_order_update() {
        let data = r#"
        {"E":1748288167010366,"O":"USER","P":"178.05","Q":"0","S":"Ask","T":1748288167009460,"V":"RejectTaker","X":"TriggerPending","Y":"20.03","Z":"0","e":"triggerPlaced","f":"GTC","i":"114575813313101824","o":"LIMIT","p":"178.15","q":"0","r":false,"s":"SOL_USDC","t":null,"z":"0"}
        "#;

        let order_update: OrderUpdate = serde_json::from_str(data).unwrap();
        assert_eq!(order_update.price.unwrap(), dec!(178.15));
        assert_eq!(order_update.trigger_price.unwrap(), dec!(178.05));
        assert_eq!(order_update.trigger_quantity.unwrap(), dec!(20.03));
        assert_eq!(order_update.quantity_in_quote.unwrap(), dec!(0));

        let data = r#"
        {"E":1748288615134547,"O":"USER","Q":"3568.3445","S":"Ask","T":1748288615133255,"V":"RejectTaker","X":"New","Z":"0","e":"orderAccepted","f":"GTC","i":"114575842681290753","o":"LIMIT","p":"178.15","q":"20.03","r":false,"s":"SOL_USDC","t":null,"z":"0"}
        "#;

        let order_update: OrderUpdate = serde_json::from_str(data).unwrap();
        assert_eq!(order_update.price.unwrap(), dec!(178.15));
        assert_eq!(order_update.trigger_price, None);
        assert_eq!(order_update.quantity_in_quote.unwrap(), dec!(3568.3445));
        assert_eq!(order_update.quantity, dec!(20.03));

        let data = r#"
        {"B":"LastPrice","E":1748289564405220,"O":"USER","P":"178.55","S":"Ask","T":1748289564404373,"V":"RejectTaker","X":"Cancelled","Y":"1","Z":"0","e":"orderCancelled","f":"GTC","i":"114575904705282048","o":"MARKET","q":"0","r":false,"s":"SOL_USDC","t":null,"z":"0"}
        "#;
        let order_update: OrderUpdate = serde_json::from_str(data).unwrap();
        assert_eq!(order_update.trigger_price.unwrap(), dec!(178.55));
    }

    #[test]
    fn test_flexible_timestamp_parsing() {
        // Test integer timestamp (from open orders API)
        let timestamp_int = super::flexible_timestamp::deserialize(serde_json::Value::Number(serde_json::Number::from(1754019359017_i64))).unwrap();
        assert_eq!(timestamp_int, 1754019359017);

        // Test string timestamp with milliseconds (from order history API)
        let timestamp_str = super::flexible_timestamp::deserialize(serde_json::Value::String("2025-07-24T04:05:48.931".to_string())).unwrap();
        // This should convert to a reasonable Unix timestamp in milliseconds
        assert!(timestamp_str > 1700000000000); // After 2023
        assert!(timestamp_str < 2000000000000); // Before 2033

        // Test string timestamp without milliseconds
        let timestamp_str_no_ms = super::flexible_timestamp::deserialize(serde_json::Value::String("2025-07-24T04:05:48".to_string())).unwrap();
        assert!(timestamp_str_no_ms > 1700000000000);
        assert!(timestamp_str_no_ms < 2000000000000);

        // Test string that looks like an integer
        let timestamp_str_int = super::flexible_timestamp::deserialize(serde_json::Value::String("1754019359017".to_string())).unwrap();
        assert_eq!(timestamp_str_int, 1754019359017);
    }

    #[test]
    fn test_historical_order_parsing() {
        // Test parsing a historical order (from /wapi/v1/history/orders)
        let historical_order_json = r#"
        {
            "clientId": null,
            "createdAt": "2025-07-24T04:05:48.931",
            "executedQuantity": "0",
            "executedQuoteQuantity": "0",
            "expiryReason": null,
            "id": "3998427932",
            "orderType": "Limit",
            "postOnly": false,
            "price": "182",
            "quantity": "0.1",
            "quoteQuantity": "18.2",
            "selfTradePrevention": "RejectTaker",
            "side": "Bid",
            "status": "Cancelled",
            "stopLossLimitPrice": null,
            "stopLossTriggerBy": null,
            "stopLossTriggerPrice": null,
            "strategyId": null,
            "symbol": "SOL_USDC",
            "systemOrderType": null,
            "takeProfitLimitPrice": null,
            "takeProfitTriggerBy": null,
            "takeProfitTriggerPrice": null,
            "timeInForce": "GTC",
            "triggerBy": null,
            "triggerPrice": null,
            "triggerQuantity": null
        }
        "#;

        let order: Order = serde_json::from_str(historical_order_json).unwrap();
        match order {
            Order::Limit(limit_order) => {
                assert_eq!(limit_order.id, "3998427932");
                assert_eq!(limit_order.symbol, "SOL_USDC");
                assert_eq!(limit_order.price, dec!(182));
                assert_eq!(limit_order.quantity, dec!(0.1));
                assert_eq!(limit_order.status, OrderStatus::Cancelled);
                assert_eq!(limit_order.quote_quantity, Some(dec!(18.2)));
                assert_eq!(limit_order.strategy_id, None);
                assert_eq!(limit_order.system_order_type, None);
                // Timestamp should be parsed correctly
                assert!(limit_order.created_at > 1700000000000);
                assert!(limit_order.created_at < 2000000000000);
            }
            _ => panic!("Expected Limit order"),
        }
    }

    #[test]
    fn test_open_order_parsing_with_integer_timestamp() {
        // Test parsing an open order (from /api/v1/orders) - simulated structure
        let open_order_json = r#"
        {
            "id": "4570246549",
            "clientId": null,
            "symbol": "SOL_USDC",
            "side": "Bid",
            "quantity": "0.1",
            "executedQuantity": "0",
            "executedQuoteQuantity": "0",
            "stopLossLimitPrice": null,
            "stopLossTriggerBy": null,
            "stopLossTriggerPrice": null,
            "takeProfitLimitPrice": null,
            "takeProfitTriggerBy": null,
            "takeProfitTriggerPrice": null,
            "price": "166.33",
            "triggerBy": null,
            "triggerPrice": null,
            "triggerQuantity": null,
            "triggeredAt": null,
            "timeInForce": "GTC",
            "relatedOrderId": null,
            "selfTradePrevention": "RejectTaker",
            "postOnly": false,
            "reduceOnly": null,
            "status": "New",
            "createdAt": 1754019359017,
            "orderType": "Limit"
        }
        "#;

        let order: Order = serde_json::from_str(open_order_json).unwrap();
        match order {
            Order::Limit(limit_order) => {
                assert_eq!(limit_order.id, "4570246549");
                assert_eq!(limit_order.symbol, "SOL_USDC");
                assert_eq!(limit_order.price, dec!(166.33));
                assert_eq!(limit_order.quantity, dec!(0.1));
                assert_eq!(limit_order.status, OrderStatus::New);
                assert_eq!(limit_order.created_at, 1754019359017);
                // Historical-only fields should be None for open orders
                assert_eq!(limit_order.quote_quantity, None);
                assert_eq!(limit_order.strategy_id, None);
                assert_eq!(limit_order.expiry_reason, None);
            }
            _ => panic!("Expected Limit order"),
        }
    }

    #[test]
    fn test_market_order_with_string_timestamp() {
        // Test market order with string timestamp
        let market_order_json = r#"
        {
            "id": "123456789",
            "clientId": null,
            "symbol": "SOL_USDC",
            "side": "Ask",
            "quantity": null,
            "executedQuantity": "1.5",
            "quoteQuantity": "300",
            "executedQuoteQuantity": "297.45",
            "stopLossLimitPrice": null,
            "stopLossTriggerBy": null,
            "stopLossTriggerPrice": null,
            "takeProfitLimitPrice": null,
            "takeProfitTriggerBy": null,
            "takeProfitTriggerPrice": null,
            "triggerBy": null,
            "triggerPrice": null,
            "triggerQuantity": null,
            "triggeredAt": null,
            "timeInForce": "IOC",
            "relatedOrderId": null,
            "selfTradePrevention": "RejectTaker",
            "reduceOnly": null,
            "status": "Filled",
            "createdAt": "2025-07-23T06:40:02.881",
            "orderType": "Market"
        }
        "#;

        let order: Order = serde_json::from_str(market_order_json).unwrap();
        match order {
            Order::Market(market_order) => {
                assert_eq!(market_order.id, "123456789");
                assert_eq!(market_order.symbol, "SOL_USDC");
                assert_eq!(market_order.executed_quantity, dec!(1.5));
                assert_eq!(market_order.quote_quantity, Some(dec!(300)));
                assert_eq!(market_order.status, OrderStatus::Filled);
                // Timestamp should be parsed correctly from string
                assert!(market_order.created_at > 1700000000000);
                assert!(market_order.created_at < 2000000000000);
            }
            _ => panic!("Expected Market order"),
        }
    }
}
