=======
Golfy
=======


main.rs
=========

Em ``main.rs``, algumas constantes e funções para inicializar
o estado do jogo são definidas. Depois, ``App`` é instanciado.


game/
=========

``levels/`` define as fases de forma declarativa, e ``swings_count/`` 
define o contador de tacadas.

Em ``systems.rs``, é definida a física do jogo: como a bola responde 
à uma tacada e como ela é refletida quando bate em uma parede. Isso 
é feito por meio de àlgebra vetorial.

O vetor velocidade da bola é calculado por meio 
das posições da bola e do cursor. A magnitude 
desse vetor é proporcional à distância dos dois.

Quando ocorre um clique, esse vetor se materializa
e o estado do jogo é atualizado.

Colisões com a parede são tratadas multiplicando por -1 
a componente horizontal quando existe um obstáculo na horizontal, 
e multiplicando por -1 a componente vertical quando existe um
obstáculo na vertical:

.. code-block:: rust

    match collision {
        Collision::Left => reflect_x = ball_velocity.x > 0.0,
        Collision::Right => reflect_x = ball_velocity.x < 0.0,
        Collision::Top => reflect_y = ball_velocity.y < 0.0,
        Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
    }

    if reflect_x {
        ball_velocity.x = -ball_velocity.x;
    }

    if reflect_y {
        ball_velocity.y = -ball_velocity.y;
    }



ui/
==========

``main_menu/`` define o menu inicial. Quando o botão play é apertado,
o estado é alterado para ``Game``, que por sua vez chama a função
``enter_game_state()`` que por sua vez irá carregar o primeiro nível.

Se ``check_ball_inside_hole()`` for verdadeiro, o próximo nível é carregado.



    
