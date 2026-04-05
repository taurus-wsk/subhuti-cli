@dataclass
class ContextPiece:
    content: str
    priority: int  # 1-10, higher is more important
    source: str  # recent_chat / project_context / history / ...


class ContextOrchestrator:
    def __init__(self, project_context, max_tokens: int = 8000):
        self.project_context = project_context
        self.max_tokens = max_tokens
        self.token_budget = {
            "recent_chat": 3000,
            "project_context": 2000,
            "relevant_history": 1500,
            "old_chat_summary": 1500
        }

    def prepare_context(self, state: AgentState, task_type: str) -> str:
        """
        主入口：准备优化后的上下文
        """
        context_pieces: List[ContextPiece] = []

        # 1. 提取最近对话
        context_pieces.extend(self._extract_recent_chat(state))

        # 2. 提取相关项目上下文
        context_pieces.extend(self._extract_project_context(state, task_type))

        # 3. 提取相关历史任务
        context_pieces.extend(self._extract_relevant_history(state))

        # 4. 排序
        context_pieces.sort(key=lambda x: x.priority, reverse=True)

        # 5. 压缩和优化
        optimized_context = self._compress_and_assemble(context_pieces)

        return optimized_context

    def _extract_recent_chat(self, state) -> List[ContextPiece]:
        """提取最近的 N 轮对话"""
        pass

    def _extract_project_context(self, state, task_type) -> List[ContextPiece]:
        """提取相关的项目上下文"""
        pass

    def _extract_relevant_history(self, state) -> List[ContextPiece]:
        """提取相关的历史任务（基于相似度）"""
        pass

    def _compress_and_assemble(self, pieces: List[ContextPiece]) -> str:
        """
        压缩并组装上下文
        - 遵守 Token 预算
        - 摘要化低优先级内容
        """
        pass

    def _summarize_old_chat(self, messages) -> str:
        """摘要化旧对话"""
        pass
