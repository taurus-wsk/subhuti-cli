from abc import ABC, abstractmethod

class BaseSkill(ABC):
    name: str
    description: str
    parameters: dict

    @abstractmethod
    def execute(self, **kwargs):
        pass