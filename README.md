# RS_ECC


Este projeto visa implementar o ECDSA (Elliptic Curve Digital Signature Algorithm) em Rust. O objetivo é criar uma implementação direta e funcional do ECDSA, abordando a geração de chaves, assinatura e verificação por meio de operações em curvas elípticas além de adquirir conhecimento prático e teórico em criptografia. 

## Equação Geral
A equação de Weierstrass refere-se a uma forma específica de equação que descreve uma curva elíptica cujo a forma geral definida sobre um corpo $K$ (como os números reais ou complexos) é dada por: $$y^2= x^3 + ax + b$$ onde $4a^3 + 27b^3$ não pode ser igual a $0$.
<br>
A equação de Weierstrass descreve uma curva elíptica no plano coordenado, e a sua forma específica é escolhida de modo a garantir que a curva tenha propriedades importantes, como a associatividade da operação de soma de pontos e a existência de um ponto de identidade. As constantes 
$a$ e $b$ afetam a localização e a inclinação da curva elíptica no plano.


## Operação de Soma

Podemos definir a operação de soma de dois pontos $P(x_1, y_1)$ e $Q(x_2, y_2)$ pertencentes a uma curva elíptica $E(K)$ como 
$$P(x_1, y_1) + Q(x_2, y_2) = R(x_3, y_3)$$

$$λ = (y₂ - y₁)(x₂ - x₁)⁻¹, P ≠ Q$$
$$λ = (3x₁² + A)(2y₁)⁻¹, P = Q$$
$$x₃ = λ² - x₁ - x₂$$
$$y₃ = λ(x₁ - x₃) - y₁$$


### Multiplicação por escalar e o problema do logaritmo discreto
A multiplicação por escalar em curvas elípticas, especialmente quando implementada com o algoritmo Double and Add, desempenha um papel crucial em algoritmos criptográficos como o ECDSA. Este processo envolve a multiplicação de um ponto na curva por um escalar (um número inteiro), resultando em um novo ponto na curva.
<br>
A segurança de algoritmos baseados em curvas elípticas, como o ECDSA repousa na dificuldade do Problema do Logaritmo Discreto em curvas elípticas. Esse problema, em termos simples, consiste em encontrar o expoente $d$ na equação $Q=d*G$, onde $G$ é um ponto base conhecido e $Q$ é um ponto qualquer na curva. O algoritmo de multiplicação por escalar é fundamental nesse contexto, já que a dificuldade de inverter essa operação é o que confere a segurança do sistema. 

### Ponto de Identidade:

A curva elíptica tem um ponto especial chamado ponto de identidade, denotado como 0. É um ponto fictício que serve como elemento neutro para a adição.

### Estrutura de Grupo: 

A soma de pontos em uma curva elíptica possui propriedades de grupo, incluindo associatividade, comutatividade e existência do elemento inverso. Essas propriedades são fundamentais para a segurança e eficiência dos algoritmos de criptografia baseados em curvas elípticas.

## ECDSA (Elliptic Curve Digital Signature Algorithm)

O ECDSA, ou Elliptic Curve Digital Signature Algorithm, é um algoritmo de assinatura digital baseado em curvas elípticas. Este algoritmo faz parte da família de algoritmos de criptografia de chave pública e é amplamente utilizado para garantir a autenticidade e a integridade de dados em sistemas de comunicação seguros.

### 1 - Geração de Chaves
Para fins didádicos chamaremos os dois agentes nesse exemplo de Alice e Bob. 
<br>Primeiramente é definida a curva $E(Z_p)$ com a ordem de $Z_p$ igual a um $q$ primo sobre a qual vamos trabalhar. E então escolhemos um elemento gerador $A \in E(Z_p)$. 
<br>
- Alice gera uma chave privada aleatória $d \in Z_p$ de forma que $0 < d < q$. 
- Depois Alice calcula a chave pública $B = d*A$ a partir de chave privada e do gerador do grupo.

### 2 - Assinatura da mensagem
- Primeiro selecionamos um $k_e$ aleatório, $0 < k_e < q$.
- $R(x_r, y_r) = K_e*A$, $r = x_r$.
- $S = (hash(message) + d*r)*K_e⁻¹\mod q$.
- $Signature = (r, s)$.

### 3 - Verificação da assinatura
- $u_1 = S⁻¹ * hash(message) \mod q$.
- $u_2 = S⁻¹* r \mod q$.
- $P(x_p, y_p) = u_1*A + u_2*B$.
- Se $x_p = r$ a assinatura está verificada.

### 4 - Prova
$$S = (hash(message) + d*r)*K_e⁻¹\mod q$$
$$k_e = S⁻¹*hash(message) + S⁻¹*d*r \mod q$$
$$K_e*A = u_1*A + u_2*d*A \mod q$$
$$K_e*A = u_1*A + u_2*B \mod q$$
$$K_e*A = R(x_r, y_r) = u_1*A + u_2*B \mod q = P(x_p, y_p) $$