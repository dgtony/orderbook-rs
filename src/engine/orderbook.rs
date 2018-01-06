
use std::time::SystemTime;

use super::domain::{Asset, Order, OrderSide, OrderType};
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
    Accepted {
        id: u64,
        order_type: OrderType,
        ts: SystemTime,
    },
    Filled {
        order_id: u64,
        side: OrderSide,
        order_type: OrderType,
        price: f64,
        qty: f64,
        ts: SystemTime,
    },
    PartiallyFilled {
        order_id: u64,
        side: OrderSide,
        order_type: OrderType,
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
    DuplicateOrderID(u64),
    NoMatch(u64),
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
        // processing result accumulator
        let mut proc_result: OrderProcessingResult = vec![];

        // validate request
        if let Err(reason) = self.order_validator.validate(&order) {
            proc_result.push(Err(FailedProcessingStep::ValidationFailed(
                String::from("reason"),
            )));
            return proc_result;
        }

        match order {
            OrderRequest::NewMarketOrder {
                order_asset,
                price_asset,
                side,
                qty,
                ts,
            } => {
                // generate new ID for order
                let order_id = self.seq.next_id();
                proc_result.push(Ok(SuccessfulProcessingStep::Accepted {
                    id: order_id,
                    order_type: OrderType::Market,
                    ts: SystemTime::now(),
                }));

                self.process_market_order(
                    &mut proc_result,
                    order_id,
                    order_asset,
                    price_asset,
                    side,
                    qty,
                    //ts,
                );
            }

            OrderRequest::NewLimitOrder {
                order_asset,
                price_asset,
                side,
                price,
                qty,
                ts,
            } => {
                let order_id = self.seq.next_id();
                proc_result.push(Ok(SuccessfulProcessingStep::Accepted {
                    id: order_id,
                    order_type: OrderType::Limit,
                    ts: SystemTime::now(),
                }));

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
    ) {
        // get copy of the current limit order
        let opposite_order_result = {
            let opposite_queue = match side {
                OrderSide::Bid => &mut self.ask_queue,
                OrderSide::Ask => &mut self.bid_queue,
            };
            opposite_queue.peek().cloned()
        };

        //let opposite_order_result = opposite_queue.peek().cloned();
        if let Some(opposite_order) = opposite_order_result {
            let matching_complete = self.order_matching(
                results,
                &opposite_order,
                order_id,
                order_asset,
                price_asset,
                OrderType::Market,
                side,
                opposite_order.price,
                qty,
            );

            if !matching_complete {
                // match the rest
                self.process_market_order(
                    results,
                    order_id,
                    order_asset,
                    price_asset,
                    side,
                    qty - opposite_order.qty,
                );
            }

        /*
            if qty < opposite_order.qty {
                // fill market and modify opposite
                let deal_time = SystemTime::now();

                // report filled market order
                results.push(Ok(SuccessfulProcessingStep::Filled {
                    order_id,
                    side,
                    order_type: OrderType::Market,
                    price: opposite_order.price,
                    qty,
                    ts: deal_time,
                }));

                // report partially filled opposite limit order
                results.push(Ok(SuccessfulProcessingStep::PartiallyFilled {
                    order_id: opposite_order.order_id,
                    side: opposite_order.side,
                    order_type: OrderType::Limit,
                    price: opposite_order.price,
                    qty,
                    ts: deal_time,
                }));

                // modify unmatched part of the limit order
                {
                    let opposite_queue = match side {
                        OrderSide::Bid => &mut self.ask_queue,
                        OrderSide::Ask => &mut self.bid_queue,
                    };
                    opposite_queue.modify_current_order(Order {
                        order_id: opposite_order.order_id,
                        order_asset,
                        price_asset,
                        side: opposite_order.side,
                        price: opposite_order.price,
                        qty: opposite_order.qty - qty,
                    });
                }

            } else if qty > opposite_order.qty {
                // partially fill market, fill opposite limit and recursively process the rest
                let deal_time = SystemTime::now();

                // report partially filled market order
                results.push(Ok(SuccessfulProcessingStep::PartiallyFilled {
                    order_id,
                    side,
                    order_type: OrderType::Market,
                    price: opposite_order.price,
                    qty: opposite_order.qty,
                    ts: deal_time,
                }));

                // report filled opposite limit order
                results.push(Ok(SuccessfulProcessingStep::Filled {
                    order_id: opposite_order.order_id,
                    side: opposite_order.side,
                    order_type: OrderType::Limit,
                    price: opposite_order.price,
                    qty: opposite_order.qty,
                    ts: deal_time,
                }));

                // remove filled limit order from the queue
                {
                    let opposite_queue = match side {
                        OrderSide::Bid => &mut self.ask_queue,
                        OrderSide::Ask => &mut self.bid_queue,
                    };
                    opposite_queue.pop();
                }

                // process the rest
                self.process_market_order(
                    results,
                    order_id,
                    order_asset,
                    price_asset,
                    side,
                    qty - opposite_order.qty,
                    //ts,
                )

            } else {
                // orders exactly match -> fill both and remove limit from queue
                let deal_time = SystemTime::now();

                // report filled market order
                results.push(Ok(SuccessfulProcessingStep::Filled {
                    order_id,
                    side,
                    order_type: OrderType::Market,
                    price: opposite_order.price,
                    qty,
                    ts: deal_time,
                }));

                // report filled opposite limit order
                results.push(Ok(SuccessfulProcessingStep::Filled {
                    order_id: opposite_order.order_id,
                    side: opposite_order.side,
                    order_type: OrderType::Limit,
                    price: opposite_order.price,
                    qty,
                    ts: deal_time,
                }));

                // remove filled limit order from the queue
                {
                    let opposite_queue = match side {
                        OrderSide::Bid => &mut self.ask_queue,
                        OrderSide::Ask => &mut self.bid_queue,
                    };
                    opposite_queue.pop();
                }
            }
            */

        } else {
            // no limit orders found
            results.push(Err(FailedProcessingStep::NoMatch(order_id)));
        }
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
        // take a look at current opposite limit order
        let opposite_order_result = {
            let opposite_queue = match side {
                OrderSide::Bid => &mut self.ask_queue,
                OrderSide::Ask => &mut self.bid_queue,
            };
            opposite_queue.peek().cloned()
        };

        if let Some(opposite_order) = opposite_order_result {
            let could_be_matched = match side {
                // verify bid/ask price overlap
                OrderSide::Bid => price >= opposite_order.price,
                OrderSide::Ask => price <= opposite_order.price,
            };

            if could_be_matched {
                // match immediately
                let matching_complete = self.order_matching(
                    results,
                    &opposite_order,
                    order_id,
                    order_asset,
                    price_asset,
                    OrderType::Limit,
                    side,
                    opposite_order.price,
                    qty,
                );

                if !matching_complete {
                    // process the rest of new limit order
                    self.process_limit_order(
                        results,
                        order_id,
                        order_asset,
                        price_asset,
                        side,
                        price,
                        qty - opposite_order.qty,
                        ts,
                    );
                }


            /*
                if qty < opposite_order.qty {
                    // fill new limit as market and modify opposite limit
                    let deal_time = SystemTime::now();

                    // report filled market order
                    results.push(Ok(SuccessfulProcessingStep::Filled {
                        order_id,
                        side,
                        order_type: OrderType::Limit,
                        price: opposite_order.price,
                        qty,
                        ts: deal_time,
                    }));

                    // report partially filled opposite limit order
                    results.push(Ok(SuccessfulProcessingStep::PartiallyFilled {
                        order_id: opposite_order.order_id,
                        side: opposite_order.side,
                        order_type: OrderType::Limit,
                        price: opposite_order.price,
                        qty,
                        ts: deal_time,
                    }));

                    // modify unmatched part of the limit order
                    {
                        let opposite_queue = match side {
                            OrderSide::Bid => &mut self.ask_queue,
                            OrderSide::Ask => &mut self.bid_queue,
                        };
                        opposite_queue.modify_current_order(Order {
                            order_id: opposite_order.order_id,
                            order_asset,
                            price_asset,
                            side: opposite_order.side,
                            price: opposite_order.price,
                            qty: opposite_order.qty - qty,
                        });
                    }

                } else if qty > opposite_order.qty {
                    // partially fill new limit order, fill opposite limit and
                    // recursively process the rest
                    let deal_time = SystemTime::now();

                    // report partially filled market order
                    results.push(Ok(SuccessfulProcessingStep::PartiallyFilled {
                        order_id,
                        side,
                        order_type: OrderType::Limit,
                        price: opposite_order.price,
                        qty: opposite_order.qty,
                        ts: deal_time,
                    }));

                    // report filled opposite limit order
                    results.push(Ok(SuccessfulProcessingStep::Filled {
                        order_id: opposite_order.order_id,
                        side: opposite_order.side,
                        order_type: OrderType::Limit,
                        price: opposite_order.price,
                        qty: opposite_order.qty,
                        ts: deal_time,
                    }));

                    // remove filled limit order from the queue
                    {
                        let opposite_queue = match side {
                            OrderSide::Bid => &mut self.ask_queue,
                            OrderSide::Ask => &mut self.bid_queue,
                        };
                        opposite_queue.pop();
                    }

                    // process the rest of new limit order
                    self.process_limit_order(
                        results,
                        order_id,
                        order_asset,
                        price_asset,
                        side,
                        price,
                        qty - opposite_order.qty,
                        ts,
                    );

                } else {
                    // orders exactly match -> fill both and remove old limit
                    let deal_time = SystemTime::now();

                    // report filled market order
                    results.push(Ok(SuccessfulProcessingStep::Filled {
                        order_id,
                        side,
                        order_type: OrderType::Limit,
                        price: opposite_order.price,
                        qty,
                        ts: deal_time,
                    }));

                    // report filled opposite limit order
                    results.push(Ok(SuccessfulProcessingStep::Filled {
                        order_id: opposite_order.order_id,
                        side: opposite_order.side,
                        order_type: OrderType::Limit,
                        price: opposite_order.price,
                        qty,
                        ts: deal_time,
                    }));

                    // remove filled limit order from the queue
                    {
                        let opposite_queue = match side {
                            OrderSide::Bid => &mut self.ask_queue,
                            OrderSide::Ask => &mut self.bid_queue,
                        };
                        opposite_queue.pop();
                    }
                }*/

            } else {
                // just insert new order in queue
                self.store_new_limit_order(
                    results,
                    order_id,
                    order_asset,
                    price_asset,
                    side,
                    price,
                    qty,
                    ts,
                );
            }

        } else {
            self.store_new_limit_order(
                results,
                order_id,
                order_asset,
                price_asset,
                side,
                price,
                qty,
                ts,
            );
        }
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
        let order_queue = match side {
            OrderSide::Bid => &mut self.bid_queue,
            OrderSide::Ask => &mut self.ask_queue,
        };

        if order_queue.amend(
            order_id,
            price,
            ts,
            Order {
                order_id,
                order_asset: self.order_asset,
                price_asset: self.price_asset,
                side,
                price,
                qty,
            },
        )
        {
            results.push(Ok(SuccessfulProcessingStep::Amended {
                id: order_id,
                price,
                qty,
                ts: SystemTime::now(),
            }));
        } else {
            results.push(Err(FailedProcessingStep::OrderNotFound(order_id)));
        }
    }


    fn process_order_cancel(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        side: OrderSide,
        ts: SystemTime,
    ) {
        let order_queue = match side {
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


    /* Helpers */

    fn store_new_limit_order(
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
        let order_queue = match side {
            OrderSide::Bid => &mut self.bid_queue,
            OrderSide::Ask => &mut self.ask_queue,
        };
        if !order_queue.insert(
            order_id,
            price,
            ts,
            Order {
                order_id,
                order_asset,
                price_asset,
                side,
                price,
                qty,
            },
        )
        {
            results.push(Err(FailedProcessingStep::DuplicateOrderID(order_id)))
        };
    }


    fn order_matching(
        &mut self,
        results: &mut OrderProcessingResult,
        opposite_order: &Order,
        order_id: u64,
        order_asset: Asset,
        price_asset: Asset,
        order_type: OrderType,
        side: OrderSide,
        price: f64,
        qty: f64,
    ) -> bool {
        // actual processing time
        let deal_time = SystemTime::now();

        // match immediately
        if qty < opposite_order.qty {
            // fill new limit as market and modify opposite limit

            // report filled new order
            results.push(Ok(SuccessfulProcessingStep::Filled {
                order_id,
                side,
                order_type,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // report partially filled opposite limit order
            results.push(Ok(SuccessfulProcessingStep::PartiallyFilled {
                order_id: opposite_order.order_id,
                side: opposite_order.side,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // modify unmatched part of the limit order
            {
                let opposite_queue = match side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                opposite_queue.modify_current_order(Order {
                    order_id: opposite_order.order_id,
                    order_asset,
                    price_asset,
                    side: opposite_order.side,
                    price: opposite_order.price,
                    qty: opposite_order.qty - qty,
                });
            }

        } else if qty > opposite_order.qty {
            // partially fill new limit order, fill opposite limit and
            // recursively process the rest

            // report partially filled market order
            results.push(Ok(SuccessfulProcessingStep::PartiallyFilled {
                order_id,
                side,
                order_type,
                price: opposite_order.price,
                qty: opposite_order.qty,
                ts: deal_time,
            }));

            // report filled opposite limit order
            results.push(Ok(SuccessfulProcessingStep::Filled {
                order_id: opposite_order.order_id,
                side: opposite_order.side,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                qty: opposite_order.qty,
                ts: deal_time,
            }));

            // remove filled limit order from the queue
            {
                let opposite_queue = match side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                opposite_queue.pop();
            }

            // matching incomplete
            return false;

        } else {
            // orders exactly match -> fill both and remove old limit

            // report filled market order
            results.push(Ok(SuccessfulProcessingStep::Filled {
                order_id,
                side,
                order_type,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // report filled opposite limit order
            results.push(Ok(SuccessfulProcessingStep::Filled {
                order_id: opposite_order.order_id,
                side: opposite_order.side,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                qty,
                ts: deal_time,
            }));

            // remove filled limit order from the queue
            {
                let opposite_queue = match side {
                    OrderSide::Bid => &mut self.ask_queue,
                    OrderSide::Ask => &mut self.bid_queue,
                };
                opposite_queue.pop();
            }
        }

        // complete matching
        true
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
            Err(FailedProcessingStep) => {}
            _ => panic!("asd"),
        }
    }

}
