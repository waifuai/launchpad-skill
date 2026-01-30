use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::Hash;

// ============================================================================
// AI Timeline Configuration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AITimelinePhase {
    pub year: i32,
    pub quarter: Option<i32>,
    pub name: &'static str,
    pub hard_money_multiplier: f64,
    pub ai_utility_multiplier: f64,
    pub protocol_utility_multiplier: f64,
}

pub static AI_TIMELINE_PHASES: &[AITimelinePhase] = &[
    AITimelinePhase {
        year: 2024,
        quarter: None,
        name: "Pre-AI Baseline",
        hard_money_multiplier: 25.0,
        ai_utility_multiplier: 3.0,
        protocol_utility_multiplier: 10.0,
    },
    AITimelinePhase {
        year: 2025,
        quarter: Some(2),
        name: "Personal Architect",
        hard_money_multiplier: 15.0,
        ai_utility_multiplier: 8.0,
        protocol_utility_multiplier: 12.0,
    },
    AITimelinePhase {
        year: 2025,
        quarter: Some(4),
        name: "Global Acceleration Accord",
        hard_money_multiplier: 10.0,
        ai_utility_multiplier: 15.0,
        protocol_utility_multiplier: 15.0,
    },
    AITimelinePhase {
        year: 2026,
        quarter: None,
        name: "Creative Renaissance",
        hard_money_multiplier: 5.0,
        ai_utility_multiplier: 25.0,
        protocol_utility_multiplier: 18.0,
    },
    AITimelinePhase {
        year: 2027,
        quarter: None,
        name: "Agentic (Waifu Aligner)",
        hard_money_multiplier: 2.0,
        ai_utility_multiplier: 50.0,
        protocol_utility_multiplier: 25.0,
    },
];

impl AITimelinePhase {
    pub fn for_date(year: i32, month: u32) -> Self {
        let mut current_best = AI_TIMELINE_PHASES[0].clone();
        for phase in AI_TIMELINE_PHASES.iter() {
            if phase.year < year {
                current_best = phase.clone();
            } else if phase.year == year {
                if let Some(q) = phase.quarter {
                    let current_quarter = ((month - 1) / 3) + 1;
                    if current_quarter >= q as u32 {
                        current_best = phase.clone();
                    }
                } else {
                    current_best = phase.clone();
                }
            }
        }
        current_best
    }
}

// ============================================================================
// AI Evolution Categories
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AIEvolutionCategory {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "passive_utility")]
    PassiveUtility,
    #[serde(rename = "ai_enabled")]
    AIEnabled,
    #[serde(rename = "ai_native")]
    AINative,
    #[serde(rename = "ai_evolving")]
    AIEvolving,
}

impl AIEvolutionCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            AIEvolutionCategory::Static => "Static (Cannot Evolve)",
            AIEvolutionCategory::PassiveUtility => "Passive Utility",
            AIEvolutionCategory::AIEnabled => "AI-Enabled",
            AIEvolutionCategory::AINative => "AI-Native",
            AIEvolutionCategory::AIEvolving => "AI-Evolving",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AIEvolutionCategory::Static => "Static assets that cannot adapt to AI acceleration.",
            AIEvolutionCategory::PassiveUtility => "Standard utility tokens with limited AI integration.",
            AIEvolutionCategory::AIEnabled => "Assets that can integrate AI capabilities.",
            AIEvolutionCategory::AINative => "Built specifically for AI ecosystems.",
            AIEvolutionCategory::AIEvolving => "Self-modifying protocols using AI.",
        }
    }

    pub fn baseline_multiplier(&self) -> f64 {
        match self {
            AIEvolutionCategory::Static => 25.0,
            AIEvolutionCategory::PassiveUtility => 10.0,
            AIEvolutionCategory::AIEnabled => 5.0,
            AIEvolutionCategory::AINative => 3.0,
            AIEvolutionCategory::AIEvolving => 3.0,
        }
    }

    pub fn default() -> Self {
        AIEvolutionCategory::PassiveUtility
    }
}

// ============================================================================
// Token Type Classification
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum TokenType {
    #[serde(rename = "fiat_pegged")]
    FiatPegged,
    #[serde(rename = "commodity_backed")]
    CommodityBacked,
    #[serde(rename = "hard_money")]
    HardMoney,
    #[serde(rename = "protocol_utility")]
    ProtocolUtility,
    #[serde(rename = "real_yield")]
    RealYield,
    #[serde(rename = "wrapped_bridge")]
    WrappedBridge,
    #[serde(rename = "liquid_staking")]
    LiquidStaking,
    #[serde(rename = "governance")]
    Governance,
    #[serde(rename = "meme")]
    Meme,
    #[serde(rename = "stableyield")]
    StableYield,
}

impl TokenType {
    pub fn display_name(&self) -> &'static str {
        match self {
            TokenType::FiatPegged => "Fiat-Pegged Stablecoin",
            TokenType::CommodityBacked => "Commodity-Backed",
            TokenType::HardMoney => "Hard Money (Store of Value)",
            TokenType::ProtocolUtility => "Protocol Utility Token",
            TokenType::RealYield => "Real Yield Asset",
            TokenType::WrappedBridge => "Wrapped/Bridged Asset",
            TokenType::LiquidStaking => "Liquid Staking Token",
            TokenType::Governance => "Governance Token",
            TokenType::Meme => "Meme/Speculative",
            TokenType::StableYield => "Stable Yield Token",
        }
    }

    pub fn base_multiplier_range(&self) -> (f64, f64) {
        match self {
            TokenType::FiatPegged => (0.01, 0.10),
            TokenType::Meme => (0.05, 0.50),
            TokenType::Governance => (0.20, 1.50),
            TokenType::StableYield => (0.20, 1.50),
            TokenType::CommodityBacked => (50.0, 100.0),
            TokenType::HardMoney => (20.0, 50.0),
            TokenType::ProtocolUtility => (10.0, 25.0),
            TokenType::RealYield => (5.0, 10.0),
            TokenType::WrappedBridge => (15.0, 40.0),
            TokenType::LiquidStaking => (8.0, 20.0),
        }
    }

    pub fn default() -> Self {
        TokenType::ProtocolUtility
    }
}

// ============================================================================
// Risk Class
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum RiskClass {
    #[serde(rename = "Class A (Real Yield)")]
    ClassA,
    #[serde(rename = "Class B (Systemic)")]
    ClassB,
    #[serde(rename = "Class C (Venture Risk)")]
    ClassC,
    #[serde(rename = "Class D (Speculative)")]
    ClassD,
}

impl RiskClass {
    pub fn display_name(&self) -> &'static str {
        match self {
            RiskClass::ClassA => "Class A (Real Yield)",
            RiskClass::ClassB => "Class B (Systemic)",
            RiskClass::ClassC => "Class C (Venture Risk)",
            RiskClass::ClassD => "Class D (Speculative)",
        }
    }
}

impl PartialOrd for RiskClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RiskClass {
    fn cmp(&self, other: &Self) -> Ordering {
        // Defines mapping order: A < B < C < D
        let self_order = match self {
            RiskClass::ClassA => 1,
            RiskClass::ClassB => 2,
            RiskClass::ClassC => 3,
            RiskClass::ClassD => 4,
        };
        let other_order = match other {
            RiskClass::ClassA => 1,
            RiskClass::ClassB => 2,
            RiskClass::ClassC => 3,
            RiskClass::ClassD => 4,
        };
        self_order.cmp(&other_order)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    #[serde(default = "TokenType::default")]
    pub token_type: TokenType,
    pub archetype: RiskClass,
    pub insider_score: i32,
    #[serde(default = "default_rank")]
    pub rank: i32,
    #[serde(default = "AIEvolutionCategory::default")]
    pub ai_category: AIEvolutionCategory,
    pub market_cap: f64,
}

fn default_rank() -> i32 { 9999 }

impl Token {
    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }
    pub fn get_ai_category(&self) -> AIEvolutionCategory {
        self.ai_category.clone()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenAnalysisResult {
    pub token_type_display: String,
    pub real_valuation_multiplier: f64,
    pub trading_signal: String,
    pub reasoning: String,
    pub risk_class: String,
    pub ai_timeline_factor: f64,
}
