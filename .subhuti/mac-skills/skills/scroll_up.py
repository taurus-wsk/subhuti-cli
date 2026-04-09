from .skill_base import BaseSkill
import subprocess

class ScrollUpSkill(BaseSkill):
    name = "scroll_up"
    description = "向上滚动"
    parameters = {
        "type": "object",
        "properties": {},
        "required": []
    }

    def execute(self):
        subprocess.run(["osascript", "-e", 'tell application "System Events" to key code 116'])
        return "✅ 已向上滚动"