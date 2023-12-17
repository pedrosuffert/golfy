Quickstart
============

Para rodar o projeto, você deve ter rust instalado no seu sistema:

.. code-block:: bash

    $ curl https://sh.rustup.rs -sSf | sh

Depois, clone o repositório e construa o projeto:

.. code-block:: bash

    $ git clone https://github.com/pedrosuffert/golfy

    $ cd golfy

    $ rustup target install wasm32-unknown-unknown

    $ cargo install wasm-server-runner

    $ cargo run --target wasm32-unknown-unknown

    $ open http://127.0.0.1:1334


Para gerar essa wiki:

.. code-block:: bash
    
    $ cd docs

    $ pip install -U sphinx

    $ make html

    $ open _build/html/index.html