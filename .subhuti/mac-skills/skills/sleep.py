from .skill_base import BaseSkill
import time

class SleepSkill(BaseSkill):
    name = "sleep"
    description = "等待指定秒数"
    parameters = {
        "type": "object",
        "properties": {
            "seconds": {"type": "number"}
        },
        "required": ["seconds"]
    }

    def execute(self, seconds: float):
        time.sleep(seconds)
        return f"✅ 已等待 {seconds} 秒"