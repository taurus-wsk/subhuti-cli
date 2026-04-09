from .skill_base import BaseSkill
import subprocess
import os

class OpenDesktopSkill(BaseSkill):
    name = "open_desktop"
    description = "打开桌面文件夹"
    parameters = {
        "type": "object",
        "properties": {},
        "required": []
    }

    def execute(self):
        subprocess.run(["open", os.path.expanduser("~/Desktop")])
        return "✅ 已打开桌面"