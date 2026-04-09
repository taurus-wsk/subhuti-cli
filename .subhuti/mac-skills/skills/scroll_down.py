from .skill_base import BaseSkill
import subprocess

class ScrollDownSkill(BaseSkill):
    name = "scroll_down"
    description = "向下滚动"
    parameters = {
        "type": "object",
        "properties": {},
        "required": []
    }

    def execute(self):
        subprocess.run(["osascript", "-e", 'tell application "System Events" to key code 121'])
        return "✅ 已向下滚动"