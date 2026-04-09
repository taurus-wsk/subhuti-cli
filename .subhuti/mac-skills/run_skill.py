import sys
import json
from skills import load_skills

def run_skill(skill_name: str, params: str):
    skills = load_skills()
    if skill_name not in skills:
        return f"❌ 技能不存在：{skill_name}"
    try:
        return skills[skill_name].execute(params)
    except Exception as e:
        return f"❌ 执行失败：{str(e)}"

if __name__ == "__main__":
    skill_name = sys.argv[1]
    params = sys.argv[2] if len(sys.argv) > 2 else {}
    print(run_skill(skill_name, params))