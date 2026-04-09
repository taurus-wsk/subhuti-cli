from .skill_base import BaseSkill
import subprocess
import os

class ScreenshotSkill(BaseSkill):
    name = "screenshot"
    description = "截图并保存，默认保存到桌面"
    parameters = {
        "type": "object",
        "properties": {
            "path": {"type": "string"}
        },
        "required": []
    }

    def execute(self, path=None):
        path = path or os.path.expanduser("~/Desktop/screenshot.png")
        subprocess.run(["screencapture", "-x", path])
        return f"✅ 截图已保存：{path}"