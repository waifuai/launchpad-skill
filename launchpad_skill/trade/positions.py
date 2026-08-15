from __future__ import annotations

import json

from ..pricing import AIEvolutionCategory, RiskClass, Token, TokenType
from ..utils import generate_memo


def _get_max_leverage(archetype: RiskClass) -> float:
    return {
        RiskClass.CLASS_A: 10.0,
        RiskClass.CLASS_B: 5.0,
        RiskClass.CLASS_C: 2.0,
        RiskClass.CLASS_D: 1.0,
    }[archetype]


def open_position(symbol: str, direction: str, amount: int, leverage: float) -> str:
    # Default to Class C for Launchpad
    token = Token(
        symbol=symbol,
        name=f"{symbol} DAO",
        token_type=TokenType.PROTOCOL_UTILITY,
        archetype=RiskClass.CLASS_C,
        insider_score=50,
        rank=9999,
        ai_category=AIEvolutionCategory.AI_ENABLED,
        market_cap=1_000_000.0,
    )

    max_lev = _get_max_leverage(token.archetype)

    if leverage > max_lev:
        # Return a mock error as JSON for consistent output
        error = {
            "error": "Leverage too high",
            "max_leverage": max_lev,
            "risk_class": token.archetype.display_name(),
        }
        return json.dumps(error)

    memo_data = {
        "symbol": symbol,
        "direction": direction,
        "amount": amount,
        "leverage": leverage,
        "type": "derivative_trade",
    }

    return generate_memo("TRADE_OPEN", memo_data)


def liquidate_position(symbol: str, user: str) -> str:
    memo_data = {
        "symbol": symbol,
        "user": user,
        "reason": "undercollateralized",
        "type": "liquidation",
    }
    return generate_memo("TRADE_LIQUIDATE", memo_data)
