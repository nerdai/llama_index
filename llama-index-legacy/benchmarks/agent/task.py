from typing import Callable, List

from llama_index.legacy.bridge.pydantic import BaseModel
from llama_index.legacy.tools.types import BaseTool


class Task(BaseModel):
    message: str
    expected_response: str
    tools: List[BaseTool]
    eval_fn: Callable[[str, str], bool]

    class Config:
        arbitrary_types_allowed = True
