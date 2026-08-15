from __future__ import annotations

import argparse
import json
import sys

from .debt import bonds
from .equity import actions as equity_actions
from .feed import narrative
from .trade import positions


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="ls", description="Launchpad Skill - Agentic Company Management CLI")
    parser.add_argument("--json", action="store_true", help="Output in JSON format")

    subparsers = parser.add_subparsers(dest="command", required=True)

    equity_parser = subparsers.add_parser("equity")
    equity_sub = equity_parser.add_subparsers(dest="equity_action", required=True)

    equity_issue = equity_sub.add_parser("issue")
    equity_issue.add_argument("--name", required=True)
    equity_issue.add_argument("--symbol", required=True)
    equity_issue.add_argument("--supply", required=True, type=int)

    equity_swap = equity_sub.add_parser("swap")
    equity_swap.add_argument("--from", dest="from_", required=True)
    equity_swap.add_argument("--to", required=True)
    equity_swap.add_argument("--amount", required=True, type=int)

    equity_status = equity_sub.add_parser("status")
    equity_status.add_argument("--symbol", required=True)

    debt_parser = subparsers.add_parser("debt")
    debt_sub = debt_parser.add_subparsers(dest="debt_action", required=True)

    debt_issue_bond = debt_sub.add_parser("issue-bond")
    debt_issue_bond.add_argument("--symbol", required=True)
    debt_issue_bond.add_argument("--principal", required=True, type=int)

    debt_buy_bond = debt_sub.add_parser("buy-bond")
    debt_buy_bond.add_argument("--symbol", required=True)
    debt_buy_bond.add_argument("--amount", required=True, type=int)

    debt_yield = debt_sub.add_parser("yield")
    debt_yield.add_argument("--symbol", required=True)

    trade_parser = subparsers.add_parser("trade")
    trade_sub = trade_parser.add_subparsers(dest="trade_action", required=True)

    trade_long = trade_sub.add_parser("long")
    trade_long.add_argument("--symbol", required=True)
    trade_long.add_argument("--amount", required=True, type=int)
    trade_long.add_argument("--leverage", required=True, type=float)

    trade_short = trade_sub.add_parser("short")
    trade_short.add_argument("--symbol", required=True)
    trade_short.add_argument("--amount", required=True, type=int)
    trade_short.add_argument("--leverage", required=True, type=float)

    trade_liquidate = trade_sub.add_parser("liquidate")
    trade_liquidate.add_argument("--symbol", required=True)
    trade_liquidate.add_argument("--user", required=True)

    feed_parser = subparsers.add_parser("feed")
    feed_parser.add_argument("address")

    return parser


def main(argv: list[str] | None = None) -> None:
    parser = _build_parser()
    args = parser.parse_args(argv)

    if args.command == "equity":
        if args.equity_action == "issue":
            result = equity_actions.issue_equity(args.name, args.symbol, args.supply)
        elif args.equity_action == "swap":
            result = equity_actions.swap_equity(args.from_, args.to, args.amount)
        else:  # status
            result = json.dumps(equity_actions.check_status(args.symbol))
    elif args.command == "debt":
        if args.debt_action == "issue-bond":
            result = bonds.issue_bond(args.symbol, args.principal)
        elif args.debt_action == "buy-bond":
            result = bonds.buy_bond(args.symbol, args.amount)
        else:  # yield
            result = json.dumps(bonds.check_yield(args.symbol))
    elif args.command == "trade":
        if args.trade_action == "long":
            result = positions.open_position(args.symbol, "LONG", args.amount, args.leverage)
        elif args.trade_action == "short":
            result = positions.open_position(args.symbol, "SHORT", args.amount, args.leverage)
        else:  # liquidate
            result = positions.liquidate_position(args.symbol, args.user)
    else:  # feed
        result = json.dumps(narrative.get_company_narrative(args.address))

    if args.json:
        # Ensure result is valid JSON, or wrap it
        try:
            json_val = json.loads(result)
            print(json.dumps(json_val, indent=2))
        except json.JSONDecodeError:
            # If string is not JSON, wrap it
            print(json.dumps({"output": result}))
    else:
        print(result)


if __name__ == "__main__":
    main(sys.argv[1:])
