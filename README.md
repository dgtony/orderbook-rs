# Order matching engine (orderbook)

Project is just a basic order-matching engine (orderbook), created especially for learning Rust and internals of trading systems.

Each instance of orderbook is a single-threaded reactive module for the certain currency pair. It consumes orders and return vector of events, generated during processing.

Supported features:

* market orders
* limit orders
* amending limit order price/quantity
* cancelling limit order
* partial filling


## Usage
Full example code could be found in `bin/example.rs`. Here is event log created in processing test orders:

```
Order => NewLimitOrder { order_asset: BTC, price_asset: USD, side: Bid, price: 0.98, qty: 5.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859954000 } }
Processing => [Ok(Accepted { id: 1, order_type: Limit, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860016000 } })]
Spread => not available

Order => NewLimitOrder { order_asset: BTC, price_asset: USD, side: Ask, price: 1.02, qty: 1.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859954000 } }
Processing => [Ok(Accepted { id: 2, order_type: Limit, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860064000 } })]
Spread => bid: 0.98, ask: 1.02

Order => AmendOrder { id: 1, side: Bid, price: 0.99, qty: 4.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859954000 } }
Processing => [Ok(Amended { id: 1, price: 0.99, qty: 4.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860094000 } })]
Spread => bid: 0.99, ask: 1.02

Order => NewLimitOrder { order_asset: BTC, price_asset: USD, side: Bid, price: 1.01, qty: 0.4, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859955000 } }
Processing => [Ok(Accepted { id: 3, order_type: Limit, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860119000 } })]
Spread => bid: 1.01, ask: 1.02

Order => NewLimitOrder { order_asset: BTC, price_asset: USD, side: Ask, price: 1.03, qty: 0.5, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859955000 } }
Processing => [Ok(Accepted { id: 4, order_type: Limit, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860155000 } })]
Spread => bid: 1.01, ask: 1.02

Order => NewMarketOrder { order_asset: BTC, price_asset: USD, side: Bid, qty: 1.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859955000 } }
Processing => [Ok(Accepted { id: 5, order_type: Market, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860180000 } }), Ok(Filled { order_id: 5, side: Bid, order_type: Market, price: 1.02, qty: 1.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860183000 } }), Ok(Filled { order_id: 2, side: Ask, order_type: Limit, price: 1.02, qty: 1.0, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860183000 } })]
Spread => bid: 1.01, ask: 1.03

Order => NewLimitOrder { order_asset: BTC, price_asset: USD, side: Ask, price: 1.05, qty: 0.5, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859955000 } }
Processing => [Ok(Accepted { id: 6, order_type: Limit, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860248000 } })]
Spread => bid: 1.01, ask: 1.03

Order => CancelOrder { id: 4, side: Ask }
Processing => [Ok(Cancelled { id: 4, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860291000 } })]
Spread => bid: 1.01, ask: 1.05

Order => NewLimitOrder { order_asset: BTC, price_asset: USD, side: Bid, price: 1.06, qty: 0.6, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 859955000 } }
Processing => [Ok(Accepted { id: 7, order_type: Limit, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860320000 } }), Ok(PartiallyFilled { order_id: 7, side: Bid, order_type: Limit, price: 1.05, qty: 0.5, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860325000 } }), Ok(Filled { order_id: 6, side: Ask, order_type: Limit, price: 1.05, qty: 0.5, ts: SystemTime { tv_sec: 1516040690, tv_nsec: 860325000 } })]
Spread => not available
```
