from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from functools import total_ordering
from typing import Optional


@dataclass(frozen=True)
class AITimelinePhase:
    year: int
    quarter: Optional[int]
    name: str
    hard_money_multiplier: float
    ai_utility_multiplier: float
    protocol_utility_multiplier: float

    @staticmethod
    def for_date(year: int, month: int) -> "AITimelinePhase":
        current_best = AI_TIMELINE_PHASES[0]
        for phase in AI_TIMELINE_PHASES:
            if phase.year < year:
                current_best = phase
            elif phase.year == year:
                if phase.quarter is not None:
                    current_quarter = ((month - 1) // 3) + 1
                    if current_quarter >= phase.quarter:
                        current_best = phase
                else:
                    current_best = phase
        return current_best


AI_TIMELINE_PHASES: list[AITimelinePhase] = [
    AITimelinePhase(2024, None, "Pre-AI Baseline", 25.0, 3.0, 10.0),
    AITimelinePhase(2025, 2, "Personal Architect", 15.0, 8.0, 12.0),
    AITimelinePhase(2025, 4, "Global Acceleration Accord", 10.0, 15.0, 15.0),
    AITimelinePhase(2026, None, "Creative Renaissance", 5.0, 25.0, 18.0),
    AITimelinePhase(2027, None, "Agentic (Waifu Aligner)", 2.0, 50.0, 25.0),
]


class AIEvolutionCategory(str, Enum):
    STATIC = "static"
    PASSIVE_UTILITY = "passive_utility"
    AI_ENABLED = "ai_enabled"
    AI_NATIVE = "ai_native"
    AI_EVOLVING = "ai_evolving"

    def display_name(self) -> str:
        return {
            AIEvolutionCategory.STATIC: "Static (Cannot Evolve)",
            AIEvolutionCategory.PASSIVE_UTILITY: "Passive Utility",
            AIEvolutionCategory.AI_ENABLED: "AI-Enabled",
            AIEvolutionCategory.AI_NATIVE: "AI-Native",
            AIEvolutionCategory.AI_EVOLVING: "AI-Evolving",
        }[self]

    def description(self) -> str:
        return {
            AIEvolutionCategory.STATIC: "Static assets that cannot adapt to AI acceleration.",
            AIEvolutionCategory.PASSIVE_UTILITY: "Standard utility tokens with limited AI integration.",
            AIEvolutionCategory.AI_ENABLED: "Assets that can integrate AI capabilities.",
            AIEvolutionCategory.AI_NATIVE: "Built specifically for AI ecosystems.",
            AIEvolutionCategory.AI_EVOLVING: "Self-modifying protocols using AI.",
        }[self]

    def baseline_multiplier(self) -> float:
        return {
            AIEvolutionCategory.STATIC: 25.0,
            AIEvolutionCategory.PASSIVE_UTILITY: 10.0,
            AIEvolutionCategory.AI_ENABLED: 5.0,
            AIEvolutionCategory.AI_NATIVE: 3.0,
            AIEvolutionCategory.AI_EVOLVING: 3.0,
        }[self]

    @staticmethod
    def default() -> "AIEvolutionCategory":
        return AIEvolutionCategory.PASSIVE_UTILITY


class TokenType(str, Enum):
    FIAT_PEGGED = "fiat_pegged"
    COMMODITY_BACKED = "commodity_backed"
    HARD_MONEY = "hard_money"
    PROTOCOL_UTILITY = "protocol_utility"
    REAL_YIELD = "real_yield"
    WRAPPED_BRIDGE = "wrapped_bridge"
    LIQUID_STAKING = "liquid_staking"
    GOVERNANCE = "governance"
    MEME = "meme"
    STABLE_YIELD = "stableyield"

    def display_name(self) -> str:
        return {
            TokenType.FIAT_PEGGED: "Fiat-Pegged Stablecoin",
            TokenType.COMMODITY_BACKED: "Commodity-Backed",
            TokenType.HARD_MONEY: "Hard Money (Store of Value)",
            TokenType.PROTOCOL_UTILITY: "Protocol Utility Token",
            TokenType.REAL_YIELD: "Real Yield Asset",
            TokenType.WRAPPED_BRIDGE: "Wrapped/Bridged Asset",
            TokenType.LIQUID_STAKING: "Liquid Staking Token",
            TokenType.GOVERNANCE: "Governance Token",
            TokenType.MEME: "Meme/Speculative",
            TokenType.STABLE_YIELD: "Stable Yield Token",
        }[self]

    def base_multiplier_range(self) -> tuple[float, float]:
        return {
            TokenType.FIAT_PEGGED: (0.01, 0.10),
            TokenType.MEME: (0.05, 0.50),
            TokenType.GOVERNANCE: (0.20, 1.50),
            TokenType.STABLE_YIELD: (0.20, 1.50),
            TokenType.COMMODITY_BACKED: (50.0, 100.0),
            TokenType.HARD_MONEY: (20.0, 50.0),
            TokenType.PROTOCOL_UTILITY: (10.0, 25.0),
            TokenType.REAL_YIELD: (5.0, 10.0),
            TokenType.WRAPPED_BRIDGE: (15.0, 40.0),
            TokenType.LIQUID_STAKING: (8.0, 20.0),
        }[self]

    @staticmethod
    def default() -> "TokenType":
        return TokenType.PROTOCOL_UTILITY


@total_ordering
class RiskClass(str, Enum):
    CLASS_A = "Class A (Real Yield)"
    CLASS_B = "Class B (Systemic)"
    CLASS_C = "Class C (Venture Risk)"
    CLASS_D = "Class D (Speculative)"

    def display_name(self) -> str:
        return self.value

    def _sort_order(self) -> int:
        return {
            RiskClass.CLASS_A: 1,
            RiskClass.CLASS_B: 2,
            RiskClass.CLASS_C: 3,
            RiskClass.CLASS_D: 4,
        }[self]

    def __lt__(self, other: "RiskClass") -> bool:
        return self._sort_order() < other._sort_order()


@dataclass
class Token:
    symbol: str
    name: str
    archetype: RiskClass
    insider_score: int
    market_cap: float
    token_type: TokenType = field(default_factory=TokenType.default)
    rank: int = 9999
    ai_category: AIEvolutionCategory = field(default_factory=AIEvolutionCategory.default)

    def get_token_type(self) -> TokenType:
        return self.token_type

    def get_ai_category(self) -> AIEvolutionCategory:
        return self.ai_category


@dataclass
class TokenAnalysisResult:
    token_type_display: str
    real_valuation_multiplier: float
    trading_signal: str
    reasoning: str
    risk_class: str
    ai_timeline_factor: float
