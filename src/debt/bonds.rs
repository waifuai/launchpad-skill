use crate::utils::generate_memo;
use crate::pricing::{ReasoningPricer, Token, TokenType, RiskClass, AIEvolutionCategory};
use serde_json::json;

pub fn issue_bond(symbol: &str, principal: u64) -> String {
    let pricer = ReasoningPricer::new();
    
    // Default to Class C for new launchpad projects
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
    
    let analysis = pricer.analyze_token(&token);
    
    // Interest Rate Logic
    // Base Rate 5%
    let base_rate = 0.05;
    
    let risk_premium = match token.archetype {
        RiskClass::ClassA => 0.00,
        RiskClass::ClassB => 0.03,
        RiskClass::ClassC => 0.10,
        RiskClass::ClassD => 0.25,
    };
    
    let ai_impact = if analysis.ai_timeline_factor > 1.2 {
        -0.02 // Discount for high growth potential
    } else if analysis.ai_timeline_factor < 0.8 {
        0.05 // Premium for static/dying assets
    } else {
        0.0
    };
    
    let final_rate = f64::max(base_rate + risk_premium + ai_impact, 0.01); // Min 1%
    
    let memo_data = json!({
        "symbol": symbol,
        "principal": principal,
        "coupon_rate": format!("{:.2}%", final_rate * 100.0),
        "risk_class": analysis.risk_class,
        "ai_factor": analysis.ai_timeline_factor,
        "type": "bond_issuance"
    });
    
    generate_memo("DEBT_ISSUE", &memo_data)
}

pub fn buy_bond(symbol: &str, amount: u64) -> String {
    let memo_data = json!({
        "symbol": symbol,
        "amount": amount,
        "type": "bond_purchase"
    });
    generate_memo("DEBT_BUY", &memo_data)
}

pub fn check_yield(symbol: &str) -> serde_json::Value {
    // Mock yield check
    // In reality, this would query the on-chain bond state
    // Re-using calculation logic for demo consistency
    let pricer = ReasoningPricer::new();
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
    let analysis = pricer.analyze_token(&token);
    
    json!({
        "symbol": symbol,
        "current_yield": "13.0%", // Mocked based on prev logic (5+10-2=13)
        "risk_rating": analysis.risk_class
    })
}
