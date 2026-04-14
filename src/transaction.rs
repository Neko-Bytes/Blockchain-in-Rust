use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Transaction{
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

impl Transaction{
    pub fn new(sender: String, receiver: String, amount: f64) -> Self{
        Transaction{
            sender,
            receiver,
            amount,
        }
    }
}
