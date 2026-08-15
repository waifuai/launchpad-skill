from __future__ import annotations

import math

from .models import AIEvolutionCategory, AITimelinePhase, RiskClass, Token, TokenAnalysisResult, TokenType

_RISK_CLASS_MULTIPLIERS: dict[RiskClass, float] = {
    RiskClass.CLASS_A: 1.2,
    RiskClass.CLASS_B: 1.0,
    RiskClass.CLASS_C: 0.8,
    RiskClass.CLASS_D: 0.6,
}


def _rust_round(val: float) -> float:
    """Round half away from zero, matching Rust's f64::round()."""
    return math.floor(val + 0.5) if val >= 0.0 else math.ceil(val - 0.5)


class ReasoningPricer:
    def __init__(self) -> None:
        self.risk_class_multipliers = dict(_RISK_CLASS_MULTIPLIERS)

    def calculate_insider_risk_factor(self, insider_score: int) -> float:
        insider_factor = insider_score / 100.0
        return 1.0 - (insider_factor * 0.5)

    def calculate_capital_flight_factor(self, token: Token) -> float:
        if token.rank < 10:
            return 1.2
        elif token.rank < 100:
            return 0.8
        else:
            return 0.2

    def calculate_liquidity_risk_factor(self, token: Token) -> float:
        if token.symbol == "SOL":
            return 1.0
        if token.market_cap >= 1_000_000_000.0:
            return 1.0
        elif token.market_cap >= 100_000_000.0:
            return 0.95
        else:
            return 0.90

    def get_current_ai_phase(self) -> AITimelinePhase:
        return AITimelinePhase.for_date(2026, 1)

    def calculate_ai_timeline_factor(self, token: Token, phase: AITimelinePhase) -> float:
        ai_category = token.get_ai_category()
        baseline = ai_category.baseline_multiplier()

        if ai_category == AIEvolutionCategory.STATIC:
            current_multiplier = phase.hard_money_multiplier
        elif ai_category == AIEvolutionCategory.PASSIVE_UTILITY:
            current_multiplier = phase.protocol_utility_multiplier
        elif ai_category == AIEvolutionCategory.AI_ENABLED:
            base = phase.protocol_utility_multiplier
            target = phase.ai_utility_multiplier
            progress = self._get_ai_progress_weight(phase.year)
            current_multiplier = base + (target - base) * progress
        elif ai_category == AIEvolutionCategory.AI_NATIVE:
            current_multiplier = phase.ai_utility_multiplier
        else:  # AI_EVOLVING
            current_multiplier = phase.ai_utility_multiplier * 1.25

        if baseline > 0.0:
            return current_multiplier / baseline
        return 1.0

    def _get_ai_progress_weight(self, year: int) -> float:
        if year < 2025:
            return 0.0
        elif year == 2025:
            return 0.25
        elif year == 2026:
            return 0.6
        elif year >= 2027:
            return 1.0
        return 0.0

    def calculate_real_valuation_multiplier(self, token: Token) -> float:
        phase = self.get_current_ai_phase()
        return self.calculate_ai_acceleration_multiplier(token, phase)

    def calculate_ai_acceleration_multiplier(self, token: Token, phase: AITimelinePhase) -> float:
        token_type = token.get_token_type()
        min_multiplier, max_multiplier = token_type.base_multiplier_range()

        base_multiplier = (min_multiplier + max_multiplier) / 2.0
        ai_timeline_factor = self.calculate_ai_timeline_factor(token, phase)

        risk_multiplier = self.risk_class_multipliers.get(token.archetype, 1.0)
        insider_risk_factor = self.calculate_insider_risk_factor(token.insider_score)
        capital_flight_factor = self.calculate_capital_flight_factor(token)
        liquidity_risk_factor = self.calculate_liquidity_risk_factor(token)

        return (
            base_multiplier
            * ai_timeline_factor
            * risk_multiplier
            * insider_risk_factor
            * capital_flight_factor
            * liquidity_risk_factor
        )

    def generate_trading_signal(self, real_multiplier: float, token: Token) -> str:
        token_type = token.get_token_type()

        if token_type == TokenType.FIAT_PEGGED:
            return "SELL"
        elif token_type == TokenType.HARD_MONEY:
            return "BUY" if real_multiplier >= 20.0 else "HOLD"
        elif token_type == TokenType.COMMODITY_BACKED:
            return "BUY" if real_multiplier >= 30.0 else "HOLD"
        elif token_type in (TokenType.PROTOCOL_UTILITY, TokenType.LIQUID_STAKING):
            if real_multiplier >= 10.0:
                return "BUY"
            elif real_multiplier >= 3.0:
                return "HOLD"
            else:
                return "SELL"
        elif token_type == TokenType.REAL_YIELD:
            if real_multiplier >= 8.0:
                return "BUY"
            elif real_multiplier >= 4.0:
                return "HOLD"
            else:
                return "SELL"
        elif token_type == TokenType.WRAPPED_BRIDGE:
            if real_multiplier >= 15.0:
                return "BUY"
            elif real_multiplier >= 8.0:
                return "HOLD"
            else:
                return "SELL"
        elif token_type == TokenType.GOVERNANCE:
            if real_multiplier >= 2.0:
                return "BUY"
            elif real_multiplier >= 1.0:
                return "HOLD"
            else:
                return "SELL"
        elif token_type == TokenType.MEME:
            return "BUY" if real_multiplier >= 0.5 else "SELL"
        elif token_type == TokenType.STABLE_YIELD:
            return "BUY" if real_multiplier >= 2.0 else "SELL"

        raise AssertionError(f"unhandled token type: {token_type}")

    def analyze_token(self, token: Token) -> TokenAnalysisResult:
        phase = self.get_current_ai_phase()
        ai_timeline_factor = self.calculate_ai_timeline_factor(token, phase)

        real_multiplier = self.calculate_ai_acceleration_multiplier(token, phase)
        trading_signal = self.generate_trading_signal(real_multiplier, token)

        return TokenAnalysisResult(
            token_type_display=token.get_token_type().display_name(),
            real_valuation_multiplier=_rust_round(real_multiplier * 100.0) / 100.0,
            trading_signal=trading_signal,
            reasoning=f"Calculated based on AI Phase: {phase.name}",
            risk_class=token.archetype.display_name(),
            ai_timeline_factor=_rust_round(ai_timeline_factor * 100.0) / 100.0,
        )
