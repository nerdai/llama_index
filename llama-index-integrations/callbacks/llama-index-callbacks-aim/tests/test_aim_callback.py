from llama_index.callbacks.aim.base import AimCallback
from llama_index.core.callbacks.base_handler import BaseCallbackHandler


def test_arize_handler_callable():
    handler = AimCallback()
    assert isinstance(handler, BaseCallbackHandler)
