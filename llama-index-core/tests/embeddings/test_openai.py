from llama_index.embeddings.openai import OpenAIEmbedding


def test_openai_class():
    assert OpenAIEmbedding.class_name() == "OpenAIEmbedding"
