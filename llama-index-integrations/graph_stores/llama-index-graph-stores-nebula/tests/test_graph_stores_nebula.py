from llama_index.core.graph_stores.types import GraphStore
from unittest.mock import MagicMock, patch


@patch("llama_index.graph_stores.nebula.NebulaGraphStore")
def test_kuzu_graph_store(MockNebulaGraphStore: MagicMock):
    instance = MockNebulaGraphStore.return_value()
    assert isinstance(instance, GraphStore)
