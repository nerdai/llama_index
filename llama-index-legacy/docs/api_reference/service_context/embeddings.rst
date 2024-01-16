.. _Ref-Embeddings:

Embeddings
=================

Users have a few options to choose from when it comes to embeddings.

- :code:`OpenAIEmbedding`: the default embedding class. Defaults to "text-embedding-ada-002"
- :code:`HuggingFaceEmbedding`: a generic wrapper around HuggingFace's transformers models.
- :code:`OptimumEmbedding`: support for usage and creation of ONNX models from Optimum and HuggingFace.
- :code:`InstructorEmbedding`: a wrapper around Instructor embedding models.
- :code:`LangchainEmbedding`: a wrapper around Langchain's embedding models.
- :code:`GoogleUnivSentEncoderEmbedding`: a wrapper around Google's Universal Sentence Encoder.
- :code:`AdapterEmbeddingModel`: an adapter around any embedding model.


OpenAIEmbedding
===============

.. autopydantic_model:: llama_index.legacy.embeddings.openai.OpenAIEmbedding
   :members:

HuggingFaceEmbedding
====================

.. autopydantic_model:: llama_index.legacy.embeddings.huggingface.HuggingFaceEmbedding
   :members:

OptimumEmbedding
================

.. autopydantic_model:: llama_index.legacy.embeddings.huggingface_optimum.OptimumEmbedding
   :members:

InstructorEmbedding
===================

.. autopydantic_model:: llama_index.legacy.embeddings.instructor.InstructorEmbedding
   :members:

LangchainEmbedding
==================

.. autopydantic_model:: llama_index.legacy.embeddings.langchain.LangchainEmbedding
   :members:

GoogleUnivSentEncoderEmbedding
==============================

.. autopydantic_model:: llama_index.legacy.embeddings.google.GoogleUnivSentEncoderEmbedding
   :members:


.. .. automodule:: llama_index.legacy.embeddings.openai
..    :members:
..    :inherited-members:
..    :exclude-members: OAEM, OpenAIEmbeddingMode


.. We also introduce a :code:`LangchainEmbedding` class, which is a wrapper around Langchain's embedding models.
.. A full list of embeddings can be found `here <https://langchain.readthedocs.io/en/latest/reference/modules/embeddings.html>`_.

.. .. automodule:: llama_index.legacy.embeddings.langchain
..    :members:
..    :inherited-members:
