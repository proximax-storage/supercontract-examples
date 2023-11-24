pub mod blockchain;

use serde::Serialize;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    let expected_loan = quote(500);
    assert_eq!(expected_loan, 1000);

    let mut loan = Loan::new(500);

    let mut balance = loan.check_balance();
    assert_eq!(balance, 1100);

    let x = loan.pay(500);
    assert_eq!(x, 0);
    balance = loan.check_balance();
    assert_eq!(balance, 600);
    //
    let y = loan.pay(600);
    assert_eq!(y, 1);
    return y;
}

// fund
struct Loan {
    due_date: u64,
    loan_amount: u64,
    interest: u64,
    paid: u64,
    mortgage: u64, // other asset deposit
}

#[derive(Serialize)]
struct Mosaic {
    mosaic_id: u64,
    amount: u64,
}

impl Loan {
    fn new(mortgage: u64) -> Loan {
        Loan {
            due_date: blockchain::get_block_height() + 2000,
            loan_amount: mortgage*2,
            interest: mortgage*2*10/100,
            paid: 0,
            mortgage,
        }
    }

    fn check_balance(&self) -> u64 {
        return self.loan_amount + self.interest - self.paid;
    }

    fn pay(&mut self, amount: u64) -> i32 {
        self.paid = self.paid + amount;

        // fine for late payment
        if blockchain::get_block_height() > self.due_date && self.loan_amount + self.interest > 0 {
            self.interest = self.interest + self.mortgage * 10 / 100;
        }

        // finish repay loan return mortgage
        if self.loan_amount + self.interest == self.paid {
            let mut emb = blockchain::EmbeddedTransaction::default();
            emb.set_entity_type(0x4154);
            emb.set_version(3);
            let receiver = [99u8; 32];
            let mosaic_size = 1u64.to_le_bytes();
            let msg_size = 0u64.to_le_bytes();
            let mosaic = Mosaic {
                mosaic_id: 2,
                amount: self.mortgage,
            };
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
        }
        return 0;
    }
}

fn quote(mortgage: u64) -> u64 {
    return mortgage * 2;
}
