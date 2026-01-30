use crate::utils::generate_memo;
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueArgs {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
}

pub fn issue_equity(args: IssueArgs) -> String {
    let memo_data = json!({
        "symbol": args.symbol,
        "name": args.name,
        "supply": args.supply,
        "type": "equity_issuance"
    });
    
    generate_memo("EQUITY_ISSUE", &memo_data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapArgs {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

pub fn swap_equity(args: SwapArgs) -> String {
    let memo_data = json!({
        "from": args.from,
        "to": args.to,
        "amount": args.amount,
        "type": "equity_swap"
    });
    
    generate_memo("EQUITY_SWAP", &memo_data)
}

pub fn check_status(symbol: &str) -> serde_json::Value {
    // Mock status for now
    json!({
        "symbol": symbol,
        "active": true,
        "holders": 120,
        "market_cap": 1000000,
        "price": 1.5
    })
}
