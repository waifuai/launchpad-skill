use crate::utils::generate_memo;
use crate::pricing::{Token, TokenType, RiskClass, AIEvolutionCategory};
use serde_json::json;

fn get_max_leverage(archetype: &RiskClass) -> f64 {
    match archetype {
        RiskClass::ClassA => 10.0,
        RiskClass::ClassB => 5.0,
        RiskClass::ClassC => 2.0,
        RiskClass::ClassD => 1.0,
    }
}

pub fn open_position(symbol: &str, direction: &str, amount: u64, leverage: f64) -> String {
    // Default to Class C for Launchpad
    let token = Token {
        symbol: symbol.to_string(),
        name: format!("{} DAO", symbol),
        token_type: TokenType::ProtocolUtility,
        archetype: RiskClass::ClassC,
        insider_score: 50,
        rank: 9999,
        ai_category: AIEvolutionCategory::AIEnabled,
        market_cap: 1_000_000.0,
    };
    
    let max_lev = get_max_leverage(&token.archetype);
    
    if leverage > max_lev {
        // Return a mock error as JSON for consistent output
         let error = json!({
            "error": "Leverage too high",
            "max_leverage": max_lev,
            "risk_class": token.archetype.display_name()
        });
        return serde_json::to_string(&error).unwrap();
    }
    
    let memo_data = json!({
        "symbol": symbol,
        "direction": direction,
        "amount": amount,
        "leverage": leverage,
        "type": "derivative_trade"
    });
    
    generate_memo("TRADE_OPEN", &memo_data)
}

pub fn liquidate_position(symbol: &str, user: &str) -> String {
    let memo_data = json!({
        "symbol": symbol,
        "user": user,
        "reason": "undercollateralized",
        "type": "liquidation"
    });
    
    generate_memo("TRADE_LIQUIDATE", &memo_data)
}
