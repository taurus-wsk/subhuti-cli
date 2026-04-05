@dataclass
class ErrorClassification:
    error_type: str  # SYNTAX_ERROR / DEPENDENCY_ERROR / ...
    confidence: float


@dataclass
class RootCause:
    cause: str
    location: Optional[str]  # 文件/行号
    evidence: List[str]


@dataclass
class RepairSolution:
    description: str
    commands: List[str]
    risk_level: str  # low / medium / high
    success_probability: float


class ErrorDiagnosisAndRepairEngine:
    def __init__(self, project_context):
        self.project_context = project_context
        self.rollback_manager = RollbackManager()

    def diagnose_and_repair(self, skill_use: SkillUseContent) -> bool:
        # 1. 创建快照
        snapshot_id = self.rollback_manager.create_snapshot()

        # 2. 错误分类
        classification = self._classify_error(skill_use)

        # 3. 根因分析
        root_cause = self._analyze_root_cause(skill_use, classification)

        # 4. 生成修复方案
        solutions = self._generate_repair_solutions(root_cause)

        # 5. 方案排序
        sorted_solutions = self._rank_solutions(solutions)

        # 6. 尝试修复
        for solution in sorted_solutions:
            if self._execute_and_verify(solution, skill_use):
                return True  # 修复成功
            else:
                self.rollback_manager.rollback(snapshot_id)  # 回滚

        return False  # 所有方案都失败

    def _classify_error(self, skill_use) -> ErrorClassification:
        # 基于 return_code, stderr, stdout 分类
        pass

    def _analyze_root_cause(self, skill_use, classification) -> RootCause:
        # 深入分析根因
        pass

    def _generate_repair_solutions(self, root_cause) -> List[RepairSolution]:
        # 生成多个候选方案
        pass

    def _rank_solutions(self, solutions) -> List[RepairSolution]:
        # 按风险和成功率排序
        pass

    def _execute_and_verify(self, solution, original_skill_use) -> bool:
        # 执行修复
        # 验证修复是否成功
        pass
