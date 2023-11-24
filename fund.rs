pub mod blockchain;

use serde::Serialize;
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    let mut fund = Fund {
        end_date: 0,
        goal: 0,
        amount: 0,
        investors: HashMap::new(),
    };

    fund.set_end_date(2000);
    fund.set_goal(1000);
    fund.pay(500);
    assert_eq!(fund.investors.len(), 1);
    fund.pay(501);
    assert_eq!(fund.investors.len(), 2);

    // return 1 fund is valid, return 0 fund does not achieve the goal or over due data
    let x = fund.check_fund();
    return x;
}

// fund
struct Fund {
    end_date: u64,
    goal: u64,
    amount: u64,
    investors: HashMap<u64, [u8; 32]>,
}

#[derive(Serialize)]
struct Mosaic {
    mosaic_id: u64,
    amount: u64
}

impl Fund {
    pub fn get_end_date(&self) -> u64 {
        return self.end_date;
    }

    pub fn set_end_date(&mut self, end_date: u64) {
        self.end_date = end_date;
    }

    pub fn get_goal(&self) -> u64 {
        return self.goal;
    }

    pub fn set_goal(&mut self, goal: u64) {
        self.goal = goal;
    }

    pub fn get_amount(&self) -> u64 {
        return self.amount;
    }

    pub fn pay(&mut self, amount: u64) {
        self.amount = self.get_amount() + amount;
        let investor = blockchain::get_caller_public_key();
        self.investors.insert(amount, investor);
    }

    pub fn check_fund(&self) -> i32 {
        // calculate amount transfered to the contract
        let assets = blockchain::get_service_payments();
        let mut total_amount = 0;
        for item in assets {
            total_amount += item.amount;
        }

        // check fund goal and due date,  if yes set transaction && blockchain::get_block_time() <= self.get_end_date()
        if total_amount >= self.get_goal() && blockchain::get_block_height() <= self.get_end_date() {
            let mut emb = blockchain::EmbeddedTransaction::default();
            emb.set_entity_type(0x4154);
            emb.set_version(3);
            let receiver = [99u8; 32];
            let mosaic_size = 1u64.to_le_bytes();
            let msg_size = 0u64.to_le_bytes();
            let mosaic = Mosaic{ mosaic_id: 1, amount: self.amount };
            let mosaic_byte = bincode::serialize(&mosaic).unwrap();
            let mut payload: Vec<u8> = Vec::new();
            payload.extend_from_slice(&receiver);
            payload.extend_from_slice(&mosaic_size);
            payload.extend_from_slice(&msg_size);
            payload.extend_from_slice(&mosaic_byte);
            emb.set_payload(payload);
            let mut agg = blockchain::AggregateTransaction::default();
            agg.set_max_fee(10);
            agg.add_embedded_transaction(emb);
            blockchain::set_transaction(&agg);
            return 1;
        } else if blockchain::get_block_time() > self.get_end_date() {
            for it in &self.investors {
                let mut emb = blockchain::EmbeddedTransaction::default();
                emb.set_entity_type(0x4154);
                emb.set_version(3);
                let receiver = it.1;
                let mosaic_size = 1u64.to_le_bytes();
                let msg_size = 0u64.to_le_bytes();
                let mosaic = Mosaic{ mosaic_id: 1, amount: *it.0 };
                let mosaic_byte = bincode::serialize(&mosaic).unwrap();
                let mut payload: Vec<u8> = Vec::new();
                payload.extend_from_slice(receiver);
                payload.extend_from_slice(&mosaic_size);
                payload.extend_from_slice(&msg_size);
                payload.extend_from_slice(&mosaic_byte);
                emb.set_payload(payload);
                let mut agg = blockchain::AggregateTransaction::default();
                agg.set_max_fee(10);
                agg.add_embedded_transaction(emb);
                blockchain::set_transaction(&agg);
            }
            return 2;
        }else {
            return 0;
        }
    }
}

