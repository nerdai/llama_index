from typing import Any, Dict

from llama_index.core.embeddings.huggingface import HuggingFaceEmbedding
from llama_index.core.embeddings.mock_embed_model import MockEmbedding
from llama_index.core.embeddings.openai import OpenAIEmbedding
from llama_index.core.embeddings.utils import resolve_embed_model
from pytest import MonkeyPatch


def mock_hf_embeddings(*args: Any, **kwargs: Dict[str, Any]) -> Any:
    """Mock HuggingFaceEmbeddings."""
    return


def mock_openai_embeddings(*args: Any, **kwargs: Dict[str, Any]) -> Any:
    """Mock OpenAIEmbedding."""
    return


def test_resolve_embed_model(monkeypatch: MonkeyPatch) -> None:
    monkeypatch.setattr(
        "llama_index.core.embeddings.huggingface.HuggingFaceEmbedding.__init__",
        mock_hf_embeddings,
    )
    monkeypatch.setattr(
        "llama_index.core.embeddings.openai.OpenAIEmbedding.__init__",
        mock_openai_embeddings,
    )

    # Test None
    embed_model = resolve_embed_model(None)
    assert isinstance(embed_model, MockEmbedding)

    # Test str
    embed_model = resolve_embed_model("local")
    assert isinstance(embed_model, HuggingFaceEmbedding)

    # Test LCEmbeddings
    embed_model = resolve_embed_model(HuggingFaceEmbedding())
    assert isinstance(embed_model, HuggingFaceEmbedding)

    # Test BaseEmbedding
    embed_model = resolve_embed_model(OpenAIEmbedding())
    assert isinstance(embed_model, OpenAIEmbedding)
