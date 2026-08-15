from __future__ import annotations

from ..pricing import AIEvolutionCategory, ReasoningPricer, RiskClass, Token, TokenType
from ..utils import generate_memo


def _default_launchpad_token(symbol: str) -> Token:
    # Default to Class C for new launchpad projects
    return Token(
        symbol=symbol,
        name=f"{symbol} DAO",
        token_type=TokenType.PROTOCOL_UTILITY,
        archetype=RiskClass.CLASS_C,
        insider_score=50,
        rank=9999,
        ai_category=AIEvolutionCategory.AI_ENABLED,
        market_cap=1_000_000.0,
    )


def issue_bond(symbol: str, principal: int) -> str:
    pricer = ReasoningPricer()
    token = _default_launchpad_token(symbol)
    analysis = pricer.analyze_token(token)

    # Interest Rate Logic - Base Rate 5%
    base_rate = 0.05

    risk_premium = {
        RiskClass.CLASS_A: 0.00,
        RiskClass.CLASS_B: 0.03,
        RiskClass.CLASS_C: 0.10,
        RiskClass.CLASS_D: 0.25,
    }[token.archetype]

    if analysis.ai_timeline_factor > 1.2:
        ai_impact = -0.02  # Discount for high growth potential
    elif analysis.ai_timeline_factor < 0.8:
        ai_impact = 0.05  # Premium for static/dying assets
    else:
        ai_impact = 0.0

    final_rate = max(base_rate + risk_premium + ai_impact, 0.01)  # Min 1%

    memo_data = {
        "symbol": symbol,
        "principal": principal,
        "coupon_rate": f"{final_rate * 100.0:.2f}%",
        "risk_class": analysis.risk_class,
        "ai_factor": analysis.ai_timeline_factor,
        "type": "bond_issuance",
    }

    return generate_memo("DEBT_ISSUE", memo_data)


def buy_bond(symbol: str, amount: int) -> str:
    memo_data = {
        "symbol": symbol,
        "amount": amount,
        "type": "bond_purchase",
    }
    return generate_memo("DEBT_BUY", memo_data)


def check_yield(symbol: str) -> dict:
    # Mock yield check. In reality, this would query the on-chain bond state.
    # Re-using calculation logic for demo consistency.
    pricer = ReasoningPricer()
    token = _default_launchpad_token(symbol)
    analysis = pricer.analyze_token(token)

    return {
        "symbol": symbol,
        "current_yield": "13.0%",  # Mocked based on prev logic (5+10-2=13)
        "risk_rating": analysis.risk_class,
    }
