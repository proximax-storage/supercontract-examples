pub mod blockchain;

use serde::Serialize;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    let mut lottery = Lottery {
        due_date: 0,
        user_array: Vec::new(),
        amount: 0,
    };
    lottery.set_due_date(0);
    lottery.pay(1000);
    lottery.pay(1000);
    lottery.pay(1000);

    // return 1 fund is valid, return 0 fund does not achieve the goal or over due data
    let x = lottery.check_fund();
    return x;
}
struct Lottery {
    due_date: u64,
    user_array: Vec<u8>,
    amount: u64,
}
#[derive(Serialize)]
struct Mosaic {
    mosaic_id: u64,
    amount: u64,
}
impl Lottery {
    pub fn get_due_date(&self) -> u64 {
        return self.due_date;
    }
    pub fn set_due_date(&mut self, due_date: u64) {
        self.due_date = due_date;
    }
    pub fn get_amount(&self) -> u64 {
        return self.amount;
    }
    pub fn pay(&mut self, amount: u64) {
        self.amount = self.get_amount() + amount;
        let public_key = blockchain::get_caller_public_key();
        self.user_array.extend_from_slice(&public_key);
    }
    pub fn random(&self) -> [u8; 32] {
        let mut user = [0u8; 32];
        let random_number = (blockchain::get_block_generation_time() % 3) + 1;
        assert_eq!(random_number, 1);
        if random_number == 1 {
            let start_bit = 0;
            let end_bit = 32;
            user.copy_from_slice(&self.user_array[start_bit..end_bit]);
        }
        else {
            let start_bit: usize = ((random_number - 1) * 32).try_into().unwrap();
            let end_bit: usize = start_bit + 32;
            user.copy_from_slice(&self.user_array[start_bit..end_bit]);
        }
        assert_eq!(user, [0u8; 32]);
        return user;
    }

    pub fn check_fund(&self) -> i32 {
        let mut pay_amount: u64 = 0;
        let mut profit: u64 = 0;
        let assets = blockchain::get_service_payments();
        let mut total_amount = 0;
        for item in assets {
            total_amount += item.amount;
            pay_amount = 60 / 100 * total_amount;
            profit = total_amount - pay_amount;
        }
        if blockchain::get_block_height() >= self.get_due_date() {
            let creator: [u8; 32] = [23u8; 32];
            let mut emb = blockchain::EmbeddedTransaction::default();
            emb.set_entity_type(0x4154);
            emb.set_version(3);
            let mosaic_size = 1u64.to_le_bytes();
            let msg_size = 0u64.to_le_bytes();
            let mosaic = Mosaic {
                mosaic_id: 1,
                amount: profit,
            };
            let mosaic_byte = bincode::serialize(&mosaic).unwrap();
            let mut payload: Vec<u8> = Vec::new();
            payload.extend_from_slice(&creator);
            payload.extend_from_slice(&mosaic_size);
            payload.extend_from_slice(&msg_size);
            payload.extend_from_slice(&mosaic_byte);
            emb.set_payload(payload);

            let winner = self.random();
            let mut emb2 = blockchain::EmbeddedTransaction::default();
            emb2.set_entity_type(0x4154);
            emb2.set_version(3);
            let mosaic_size2 = 1u64.to_le_bytes();
            let msg_size2 = 0u64.to_le_bytes();
            let mosaic = Mosaic {
                mosaic_id: 2,
                amount: pay_amount,
            };
            let mosaic_byte2 = bincode::serialize(&mosaic).unwrap();
            let mut payload2: Vec<u8> = Vec::new();
            payload2.extend_from_slice(&winner);
            payload2.extend_from_slice(&mosaic_size2);
            payload2.extend_from_slice(&msg_size2);
            payload2.extend_from_slice(&mosaic_byte2);
            emb2.set_payload(payload2);
            let mut agg = blockchain::AggregateTransaction::default();
            agg.set_max_fee(10);
            agg.add_embedded_transaction(emb);
            agg.add_embedded_transaction(emb2);
            blockchain::set_transaction(&agg);
            return 1;
        }else {
            return 0;
        }
    }
}