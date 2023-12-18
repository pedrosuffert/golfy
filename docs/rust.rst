.. |logo| image:: _img/rust.png
   :scale: 10%

============
|logo| Rust
============

Rust é uma linguagem de programação compilada, multi-paradigma, com foco em performance
e segurança de memória, para isso abandonando o coletor de lixo e implementou
o conceito de posse e lifetime. Devido à esse fato, software escrito em Rust
é muito mais robusto contra ataques que se aproveitam de um *buffer overflow*.

Abaixo segue uma breve introdução sobre Rust:

Síntaxe
==========

A síntaxe do Rust é muito similar à de C e C++, mas foi também 
muito influenciada por linguagens funcionais.

-----------
Variáveis
-----------
Variáveis são declaradas utilizando o palavra-chave ``let``.
Por padrão, todas as variáveis declaradas são imutáveis. Para
que seja possível alterar o valor de uma variável, 
ela deve ser declarada como ``let mut``. 
Se uma variável é declarada sem ser inicializada, 
seu tipo deve ser definido. Caso contrário, seu tipo é inferido
e será imutável durante a ``lifetime`` da variável.

.. code-block:: rust

    let x = 10; // infere o tipo i32
    let mut y = 21.12; // infere o tipo f64
    let mut z = "golfy"; // infere o tipo &str

    x = 20; // erro: x não é mutável

    y = 21; // erro: o tipo de y é f64 e o tipo de '21' é i32

    z = "GOLFY"; // sucesso


--------
Funções
--------
Funções são declaradas com ``fn``. Caso não haja a expressão ``return``,
a função retornará o último valor avaliado, de forma implícita. Na 
função abaixo, o parâmetro de entrada tem tipo ``i32``,
e retorna um valor do tipo ``u64``.

.. code-block:: rust

    fn fibonacci(n: i32) -> u64 {
        match n {
            0 => panic!(""),
            1 => 1,
            2 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2),
        }
    }

Tendo influências λuncionais, Rust suporta funções anônimas, aqui
denominadas como closures:

.. code-block:: rust

    let f = |a| { a * a * a };

    println!("{}", f(5)); // 125


--------------------
Tipos Customizados
--------------------

Rust deixou de suportar classes na versão 0.4 (~2012). Em troca, 
foram implementados structs e impls. Tipos customizados são criados
por meio de uma ``struct``, e métodos para esses tipos são 
criados por meio de uma ``impl``:

.. code-block:: rust

    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn name(&self) -> &str {
            &self.name
        }

        fn age(&self) -> &u8 {
            &self.age
        }
    }

    fn main() {
        let a = Person { name: "Zé Ninguém".to_string(), age: 32 };

        println!("name: {}", a.name()); // Zé Ninguém
        println!("age: {}", a.age()); // 32
    }


Ownership
==================

Rust não possui um coletor de lixo. Em vez disso, a memória
é administrada por meio de um mecanismo de posse. Isso permite
que não haja nenhuma referência inválida; todo valor tem um dono.
Por exemplo:

.. code-block:: rust

    fn main() {
        let x = 21;
        let y = x;

        println!("x: {x}"); // erro: a posse de 21 foi passada de x para y,
                            // depois disso, x sai de escopo (sua lifetime acaba)
        println!("y: {y}");
    }

Para que isso não aconteça, o valor apontado por ``x`` deve ser copiado
para ``y``, do seguinte modo:

.. code-block:: rust

    fn main() {
        let x = 21;
        let y = x.clone(); // y se torna um clone de x

        println!("x: {x}"); // 21
        println!("y: {y}"); // 21
    }


Borrowing
============

Borrowing, em Rust, é um conceito que se relaciona diretamente
com a ideia de empréstimo de propriedade temporária de recursos, 
permitindo que partes do código acessem dados sem tomar posse 
completa deles. Esse mecanismo é fundamental para garantir a 
segurança da memória e evitar problemas como vazamento de memória 
ou referências inválidas. Exemplo:

.. code-block:: rust

    fn f(x: &i32) {
        println!("x: {x}"); // 21

        x = 42; // erro: x não é uma referência mutável
    }

    fn main() {
        let x = 21;
    
        f(&s) ;
    }

Porém, não é possivel que f() modifique o valor de x. Para isso,
é necessário passar uma referência mutável para x:

.. code-block:: rust

    fn f(x: &i32) {
        println!("x: {x}"); // 21

        x = 42;
    }

    fn main() {
        let x = 21;
    
        f(mut& s);

        println!("x: {x}"); // 42
    }

Além disso, não é possível ter mais de uma referência mutável,
para impedir uma possível *race condition* (quando um mesmo recurso
é accessado/alterado ao mesmo tempo por mais de um agente).