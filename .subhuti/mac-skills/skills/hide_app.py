from .skill_base import BaseSkill
import subprocess

class HideAppSkill(BaseSkill):
    name = "hide_app"
    description = "隐藏当前活动窗口"
    parameters = {
        "type": "object",
        "properties": {},
        "required": []
    }

    def execute(self):
        subprocess.run(["osascript", "-e", 'tell application "System Events" to keystroke "h" using command down'])
        return "✅ 已隐藏当前窗口"