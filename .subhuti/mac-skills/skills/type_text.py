from .skill_base import BaseSkill
import subprocess

class TypeTextSkill(BaseSkill):
    name = "type_text"
    description = "输入文字"
    parameters = {
        "type": "object",
        "properties": {
            "text": {"type": "string"}
        },
        "required": ["text"]
    }

    def execute(self, text: str):
        escaped = text.replace('"', '\\"')
        script = f'''tell application "System Events" to keystroke "{escaped}"'''
        subprocess.run(["osascript", "-e", script])
        return f"✅ 已输入文字：{text}"