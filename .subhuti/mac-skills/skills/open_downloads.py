from .skill_base import BaseSkill
import subprocess
import os

class OpenDownloadsSkill(BaseSkill):
    name = "open_downloads"
    description = "打开下载文件夹"
    parameters = {
        "type": "object",
        "properties": {},
        "required": []
    }

    def execute(self):
        subprocess.run(["open", os.path.expanduser("~/Downloads")])
        return "✅ 已打开下载文件夹"