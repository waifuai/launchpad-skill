use crate::pricing::models::{
    AIEvolutionCategory, AITimelinePhase, RiskClass, Token, 
    TokenAnalysisResult, TokenType,
};

#[derive(Debug, Clone)]
pub struct ReasoningPricer {
    risk_class_multipliers: std::collections::HashMap<RiskClass, f64>,
}

impl Default for ReasoningPricer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReasoningPricer {
    pub fn new() -> Self {
        let mut risk_class_multipliers = std::collections::HashMap::new();
        risk_class_multipliers.insert(RiskClass::ClassA, 1.2);
        risk_class_multipliers.insert(RiskClass::ClassB, 1.0);
        risk_class_multipliers.insert(RiskClass::ClassC, 0.8);
        risk_class_multipliers.insert(RiskClass::ClassD, 0.6);

        Self {
            risk_class_multipliers,
        }
    }

    pub fn calculate_insider_risk_factor(&self, insider_score: i32) -> f64 {
        let insider_factor = insider_score as f64 / 100.0;
        1.0 - (insider_factor * 0.5)
    }

    pub fn calculate_capital_flight_factor(&self, token: &Token) -> f64 {
        if token.rank < 10 {
            1.2
        } else if token.rank < 100 {
            0.8
        } else {
            0.2
        }
    }
    
    pub fn calculate_liquidity_risk_factor(&self, token: &Token) -> f64 {
        if token.symbol == "SOL" {
            return 1.0;
        }
        if token.market_cap >= 1_000_000_000.0 {
            1.0
        } else if token.market_cap >= 100_000_000.0 {
            0.95
        } else {
            0.90
        }
    }

    pub fn get_current_ai_phase(&self) -> AITimelinePhase {
        AITimelinePhase::for_date(2026, 1)
    }

    pub fn calculate_ai_timeline_factor(&self, token: &Token, phase: &AITimelinePhase) -> f64 {
        let ai_category = token.get_ai_category();
        let baseline = ai_category.baseline_multiplier();
        
        let current_multiplier = match ai_category {
            AIEvolutionCategory::Static => phase.hard_money_multiplier,
            AIEvolutionCategory::PassiveUtility => phase.protocol_utility_multiplier,
            AIEvolutionCategory::AIEnabled => {
                let base = phase.protocol_utility_multiplier;
                let target = phase.ai_utility_multiplier;
                let progress = self.get_ai_progress_weight(phase.year);
                base + (target - base) * progress
            }
            AIEvolutionCategory::AINative => phase.ai_utility_multiplier,
            AIEvolutionCategory::AIEvolving => phase.ai_utility_multiplier * 1.25,
        };
        
        if baseline > 0.0 {
            current_multiplier / baseline
        } else {
            1.0
        }
    }

    fn get_ai_progress_weight(&self, year: i32) -> f64 {
        match year {
            y if y < 2025 => 0.0,
            2025 => 0.25,
            2026 => 0.6,
            y if y >= 2027 => 1.0,
            _ => 0.0,
        }
    }

    pub fn calculate_real_valuation_multiplier(&self, token: &Token) -> f64 {
        let phase = self.get_current_ai_phase();
        self.calculate_ai_acceleration_multiplier(token, &phase)
    }

    pub fn calculate_ai_acceleration_multiplier(&self, token: &Token, phase: &AITimelinePhase) -> f64 {
        let token_type = token.get_token_type();
        let (min_multiplier, max_multiplier) = token_type.base_multiplier_range();
        
        let base_multiplier = (min_multiplier + max_multiplier) / 2.0;
        let ai_timeline_factor = self.calculate_ai_timeline_factor(token, phase);
        
        let risk_multiplier = self.risk_class_multipliers
            .get(&token.archetype)
            .copied()
            .unwrap_or(1.0);

        let insider_risk_factor = self.calculate_insider_risk_factor(token.insider_score);
        let capital_flight_factor = self.calculate_capital_flight_factor(token);
        let liquidity_risk_factor = self.calculate_liquidity_risk_factor(token);

        base_multiplier * ai_timeline_factor * risk_multiplier * insider_risk_factor * capital_flight_factor * liquidity_risk_factor
    }

    pub fn generate_trading_signal(&self, real_multiplier: f64, token: &Token) -> &'static str {
        let token_type = token.get_token_type();

        if matches!(token_type, TokenType::FiatPegged) {
            return "SELL";
        }

        match token_type {
            TokenType::FiatPegged => "SELL",
            TokenType::HardMoney => {
                if real_multiplier >= 20.0 { "BUY" } else { "HOLD" }
            }
            TokenType::CommodityBacked => {
                if real_multiplier >= 30.0 { "BUY" } else { "HOLD" }
            }
            TokenType::ProtocolUtility | TokenType::LiquidStaking => {
                if real_multiplier >= 10.0 { "BUY" }
                else if real_multiplier >= 3.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::RealYield => {
                if real_multiplier >= 8.0 { "BUY" }
                else if real_multiplier >= 4.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::WrappedBridge => {
                if real_multiplier >= 15.0 { "BUY" }
                else if real_multiplier >= 8.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::Governance => {
                if real_multiplier >= 2.0 { "BUY" }
                else if real_multiplier >= 1.0 { "HOLD" }
                else { "SELL" }
            }
            TokenType::Meme => {
                if real_multiplier >= 0.5 { "BUY" }
                else { "SELL" }
            }
            TokenType::StableYield => {
                if real_multiplier >= 2.0 { "BUY" }
                else { "SELL" }
            }
        }
    }

    pub fn analyze_token(&self, token: &Token) -> TokenAnalysisResult {
        let phase = self.get_current_ai_phase();
        let ai_timeline_factor = self.calculate_ai_timeline_factor(token, &phase);
        
        let real_multiplier = self.calculate_ai_acceleration_multiplier(token, &phase);
        let trading_signal = self.generate_trading_signal(real_multiplier, token);
        
        TokenAnalysisResult {
            token_type_display: token.get_token_type().display_name().to_string(),
            real_valuation_multiplier: (real_multiplier * 100.0).round() / 100.0,
            trading_signal: trading_signal.to_string(),
            reasoning: format!("Calculated based on AI Phase: {}", phase.name),
            risk_class: token.archetype.display_name().to_string(),
            ai_timeline_factor: (ai_timeline_factor * 100.0).round() / 100.0,
        }
    }
}
