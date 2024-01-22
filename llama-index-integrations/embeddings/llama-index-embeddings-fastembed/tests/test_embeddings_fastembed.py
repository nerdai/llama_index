from llama_index.core.embeddings.base import BaseEmbedding
from llama_index.embeddings.fastembed import FastEmbedEmbedding


def test_fastembed_class():
    emb = FastEmbedEmbedding()
    assert isinstance(emb, BaseEmbedding)
