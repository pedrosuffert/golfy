.. |logo| image:: _img/wasm.png
   :scale: 8%

===================
|logo| WebAssembly
===================

WebAssembly define um formato binário portátil, 
pré-compilado, que pode ser executado diretamente no 
navegador. Nasceu de uma colaboração entre a fundação Mozilla,
W3C, Apple, Google e Microsoft. 

Wasm não é escrito à mão; escreve-se o código
fonte mais de 40 LP's, como ``Rust``, C++, Go e Python, 
que é então compilado para ``wasm``. 


--------------
Casos de uso
--------------
Por ser um formato compilado, ``wasm`` apresenta
performance superior ao javascript em tarefas computacionalmente
custosas. Ele se destaca principalmente em:

* Codificação de áudio e vídeo.
* Renderização 3D.
* Simulações complexas.
* Criptografia

Casos de uso reais:

* eBay: scanner de código de barras.
* Google Earth: roda em qualquer navegador devido ao ``wasm``.
* Unity e Unreal: ambas as engines foram portadas.
* Doom 3: sua engine também foi portada. É possível rodá-lo no 
  navegador.

---------------
Especificação
---------------

WebAssembly tem como objetivos:

* Performance: executa com performance similar àquela de código nativo.
* Segurança: código é validado e executado em um ambiente isolado.
* Agnóstico ao hardware: compila em todas as aquiteturas modernas.
* Agnóstico à plataforma: roda em navegadores, ou como uma VM (similar à 
  `JVM <https://en.wikipedia.org/wiki/Java_virtual_machine>`_), ou
  em sistemas embarcados.


