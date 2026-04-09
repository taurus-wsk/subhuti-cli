from .skill_base import BaseSkill
import subprocess

class CloseAppSkill(BaseSkill):
    name = "close_app"
    description = "关闭指定应用"
    parameters = {
        "type": "object",
        "properties": {
            "app_name": {"type": "string"}
        },
        "required": ["app_name"]
    }

    def execute(self, app_name: str):
        subprocess.run(["pkill", "-f", app_name])
        return f"✅ 已关闭应用：{app_name}"