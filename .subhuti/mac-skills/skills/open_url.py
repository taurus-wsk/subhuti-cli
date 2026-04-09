from .skill_base import BaseSkill
import subprocess

class OpenUrlSkill(BaseSkill):
    name = "open_url"
    description = "打开网页 URL"
    parameters = {
        "type": "object",
        "properties": {
            "url": {"type": "string"}
        },
        "required": ["url"]
    }

    def execute(self, url: str):
        subprocess.run(["open", url])
        return f"✅ 已打开网址：{url}"