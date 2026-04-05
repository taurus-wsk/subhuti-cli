@dataclass
class FileNode:
    path: str
    file_type: str  # file / dir
    children: List["FileNode"] = field(default_factory=list)
    content: Optional[str] = None


@dataclass
class Dependency:
    name: str
    version: Optional[str]
    source: str  # requirements.txt / go.mod / ...


class ProjectContextManager:
    def __init__(self, project_root: str):
        self.project_root = project_root
        self.file_tree: Optional[FileNode] = None
        self.dependencies: List[Dependency] = []
        self._scan_timestamp: float = 0

    def scan(self, force_refresh: bool = False):
        """扫描项目上下文"""
        if not force_refresh and self._is_fresh():
            return

        self.file_tree = self._scan_file_system()
        self.dependencies = self._parse_dependencies()
        self._scan_timestamp = time.time()

    def get_file_tree(self) -> FileNode:
        """获取完整文件树"""
        return self.file_tree

    def get_file_content(self, path: str) -> Optional[str]:
        """获取文件内容"""
        pass

    def get_dependencies(self) -> List[Dependency]:
        """获取项目依赖"""
        return self.dependencies

    def file_exists(self, path: str) -> bool:
        """检查文件是否存在"""
        pass

    def _scan_file_system(self) -> FileNode:
        """递归扫描文件系统"""
        pass

    def _parse_dependencies(self) -> List[Dependency]:
        """解析依赖文件"""
        pass

    def _is_fresh(self) -> bool:
        """检查上下文是否新鲜（未过期）"""
        pass
