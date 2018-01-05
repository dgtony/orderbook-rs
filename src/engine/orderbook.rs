
use std::time::SystemTime;

use super::domain::{Asset, Order, OrderSide};
use super::orders::OrderRequest;
use super::order_queues::OrderQueue;
use super::sequence;
use super::validation::OrderRequestValidator;


const MIN_SEQUENCE_ID: u64 = 1;
const MAX_SEQUENCE_ID: u64 = 1000;
const MAX_STALLED_INDICES_IN_QUEUE: u64 = 10;
const ORDER_QUEUE_INIT_CAPACITY: usize = 500;


type OrderProcessingResult = Vec<Result<SuccessfulProcessingStep, FailedProcessingStep>>;


#[derive(Debug)]
pub enum SuccessfulProcessingStep {
    Accepted { id: u64, ts: SystemTime },
    Filled {
        opposite_order_id: u64,
        price: f64,
        qty: f64,
        ts: SystemTime,
    },
    PartiallyFilled {
        opposite_order_id: u64,
        price: f64,
        qty: f64,
        ts: SystemTime,
    },
    Amended {
        id: u64,
        price: f64,
        qty: f64,
        ts: SystemTime,
    },
    Cancelled { id: u64, ts: SystemTime },
}


#[derive(Debug)]
pub enum FailedProcessingStep {
    ValidationFailed(String),
    OrderNotFound(u64),
}


pub struct Orderbook {
    order_asset: Asset,
    price_asset: Asset,
    bid_queue: OrderQueue<Order>,
    ask_queue: OrderQueue<Order>,
    seq: sequence::TradeSequence,
    order_validator: OrderRequestValidator,
}

impl Orderbook {
    /// Create new orderbook for pair of assets
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// let mut orderbook = Orderbook::new(Asset::BTC, Asset::USD);
    /// let result = orderbook.process_order(OrderRequest::MarketOrder{  });
    /// assert_eq!(orderbook)
    /// ```
    // todo fix doc test!
    pub fn new(order_asset: Asset, price_asset: Asset) -> Self {
        Orderbook {
            order_asset,
            price_asset,
            bid_queue: OrderQueue::new(
                OrderSide::Bid,
                MAX_STALLED_INDICES_IN_QUEUE,
                ORDER_QUEUE_INIT_CAPACITY,
            ),
            ask_queue: OrderQueue::new(
                OrderSide::Ask,
                MAX_STALLED_INDICES_IN_QUEUE,
                ORDER_QUEUE_INIT_CAPACITY,
            ),
            seq: sequence::new_sequence_gen(MIN_SEQUENCE_ID, MAX_SEQUENCE_ID),
            order_validator: OrderRequestValidator::new(
                order_asset,
                price_asset,
                MIN_SEQUENCE_ID,
                MAX_SEQUENCE_ID,
            ),
        }
    }

    pub fn process_order(&mut self, order: OrderRequest) -> OrderProcessingResult {

        let mut proc_result: OrderProcessingResult = vec![];

        // validate request
        if let Err(reason) = self.order_validator.validate(&order) {
            proc_result.push(Err(FailedProcessingStep::ValidationFailed(
                String::from("reason"),
            )));
            return proc_result;
        }

        match order {
            OrderRequest::MarketOrder {
                order_asset,
                price_asset,
                side,
                qty,
                ts,
            } => {
                // generate new ID for order
                let order_id = self.seq.next_id();
                self.process_market_order(
                    &mut proc_result,
                    order_id,
                    order_asset,
                    price_asset,
                    side,
                    qty,
                    ts,
                );
            }

            OrderRequest::LimitOrder {
                order_asset,
                price_asset,
                side,
                price,
                qty,
                ts,
            } => {
                let order_id = self.seq.next_id();
                self.process_limit_order(
                    &mut proc_result,
                    order_id,
                    order_asset,
                    price_asset,
                    side,
                    price,
                    qty,
                    ts,
                );
            }

            OrderRequest::AmendOrder {
                id,
                side,
                price,
                qty,
                ts,
            } => {
                self.process_order_amend(&mut proc_result, id, side, price, qty, ts);
            }

            OrderRequest::CancelOrder { id, side, ts } => {
                self.process_order_cancel(&mut proc_result, id, side, ts);
            }
        }

        // return collected processing results
        proc_result
    }


    /* Processing logic */

    fn process_market_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        qty: f64,
        ts: SystemTime,
    ) {

        // todo

    }

    fn process_limit_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        price: f64,
        qty: f64,
        ts: SystemTime,
    ) {

        // todo

    }

    fn process_order_amend(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        side: OrderSide,
        price: f64,
        qty: f64,
        ts: SystemTime,
    ) {

        // todo

    }

    fn process_order_cancel(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        side: OrderSide,
        ts: SystemTime,
    ) {
        let mut order_queue = match side {
            OrderSide::Bid => &mut self.bid_queue,
            OrderSide::Ask => &mut self.ask_queue,
        };

        if order_queue.cancel(order_id) {
            results.push(Ok(SuccessfulProcessingStep::Cancelled {
                id: order_id,
                ts: SystemTime::now(),
            }));
        } else {
            results.push(Err(FailedProcessingStep::OrderNotFound(order_id)));
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use super::super::orders;

    #[test]
    fn simple_cancel() {

        let mut orderbook = Orderbook::new(Asset::BTC, Asset::USD);
        let request = orders::limit_order_cancel_request(1, OrderSide::Bid, SystemTime::now());
        let mut result = orderbook.process_order(request);

        //println!("result: {:?}", &result);

        assert_eq!(result.len(), 1);
        match result.pop().unwrap() {
            Err(FailedProcessingStep) => {},
            _ => panic!("asd")
        }
    }

}
