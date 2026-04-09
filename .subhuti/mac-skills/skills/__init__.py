import os
import importlib
from skills.skill_base import BaseSkill

def load_skills():
    skills = {}
    dirname = os.path.dirname(__file__)

    for file in os.listdir(dirname):
        if file.endswith(".py") and file not in ["__init__.py", "skill_base.py"]:
            module_name = file[:-3]
            mod = importlib.import_module(f".{module_name}", package="skills")

            for attr in dir(mod):
                cls = getattr(mod, attr)
                if isinstance(cls, type) and issubclass(cls, BaseSkill) and cls != BaseSkill:
                    skill = cls()
                    skills[skill.name] = skill
    return skills

def get_skill_prompt():
    skills = load_skills()
    lines = []
    for name, skill in skills.items():
        lines.append(f"- {name}: {skill.description}")
    return "\n".join(lines)