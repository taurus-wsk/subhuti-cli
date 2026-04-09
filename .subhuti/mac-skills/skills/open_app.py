from .skill_base import BaseSkill
import subprocess

class OpenAppSkill(BaseSkill):
    name = "open_app"
    description = "打开 Mac 应用，如 Microsoft Edge、微信、访达"
    parameters = {
        "type": "object",
        "properties": {
            "app_name": {"type": "string"}
        },
        "required": ["app_name"]
    }

    def execute(self, app_name: str):
        subprocess.run(["open", "-a", app_name])
        return f"✅ 已打开应用：{app_name}"