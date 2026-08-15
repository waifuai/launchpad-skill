from datetime import datetime, timezone
import json


def generate_memo(action: str, data: dict) -> str:
    memo = {
        "protocol": "launchpad-skill",
        "action": action,
        "data": data,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }
    return json.dumps(memo)
