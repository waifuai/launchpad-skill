from __future__ import annotations

from ..utils import generate_memo


def issue_equity(name: str, symbol: str, supply: int) -> str:
    memo_data = {
        "symbol": symbol,
        "name": name,
        "supply": supply,
        "type": "equity_issuance",
    }
    return generate_memo("EQUITY_ISSUE", memo_data)


def swap_equity(from_: str, to: str, amount: int) -> str:
    memo_data = {
        "from": from_,
        "to": to,
        "amount": amount,
        "type": "equity_swap",
    }
    return generate_memo("EQUITY_SWAP", memo_data)


def check_status(symbol: str) -> dict:
    # Mock status for now
    return {
        "symbol": symbol,
        "active": True,
        "holders": 120,
        "market_cap": 1000000,
        "price": 1.5,
    }
