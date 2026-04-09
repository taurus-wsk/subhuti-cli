from .skill_base import BaseSkill
import subprocess

class ClickSkill(BaseSkill):
    name = "click"
    description = "鼠标点击指定坐标，格式：x,y"
    parameters = {
        "type": "object",
        "properties": {
            "position": {"type": "string"}
        },
        "required": ["position"]
    }

    def execute(self, position: str):
        x, y = position.split(",")
        script = f'''tell application "System Events" to click at {{x:{x}, y:{y}}}'''
        subprocess.run(["osascript", "-e", script])
        return f"✅ 已点击坐标：{position}"