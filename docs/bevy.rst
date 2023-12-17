========
Bevy
========

Bevy é um motor de jogo de código aberto escrito
em Rust. Ele foi projetado para ser um motor rápido, 
flexível e fácil de usar para a construção de jogos 
2D e 3D. O Bevy tem como objetivo aproveitar as 
vantagens da linguagem de programação Rust,
como desempenho, segurança e concorrência, para
fornecer um framework moderno de desenvolvimento de 
jogos.

* Arquitetura de Entidade-Componente-Sistema (ECS):
    O Bevy emprega uma arquitetura de Entidade-Componente-Sistema, 
    um padrão de design comumente usado no desenvolvimento de 
    jogos. Essa arquitetura ajuda a gerenciar o estado e o 
    comportamento do jogo, organizando entidades 
    (objetos do jogo) em componentes e sistemas. 
    Isso pode levar à execução eficiente e paralela do código.

* Modularidade e Extensibilidade:
    O Bevy é projetado para ser modular e extensível, 
    permitindo que os desenvolvedores usem apenas os 
    componentes e sistemas necessários para seus jogos 
    específicos. Essa modularidade promove a reutilização 
    de código e facilita a construção e a manutenção de 
    sistemas de jogos complexos.

* Sem Estado Global:
    O Bevy evita o uso de estado global, o que pode simplificar 
    o processo de desenvolvimento e tornar a base de código 
    mais manutenível. Essa escolha de design está alinhada com 
    o sistema de *posse* do Rust e contribui para a escrita
    de código robusto e escalável.

