# expression_tree

Parser de descida recursiva em Rust que lê expressões aritméticas, monta a **árvore sintática** (AST), desenha essa árvore no terminal e calcula o resultado — tratando divisão por zero e overflow sem travar o programa.

Projeto de estudo, feito para entender na prática como funcionam parsers, precedência de operadores e árvores de expressão.

## Funcionalidades

- Operadores suportados: `+`, `-`, `*`, `/`, `%` (resto), negação unária (`-x`) e parênteses
- Respeita a precedência e a associatividade usuais (`*`, `/` e `%` antes de `+` e `-`, avaliação da esquerda para a direita)
- Mostra a expressão em **forma legível**, apenas com os parênteses realmente necessários
- Desenha a **árvore sintática** no terminal
- **Avalia** o resultado com aritmética verificada (`checked_*`): divisão por zero e overflow viram mensagem de erro, não pânico
- Mensagens de erro para entradas inválidas

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) com suporte à edição 2024 (Rust 1.85 ou mais recente)

## Como rodar

```bash
git clone https://github.com/lucacer10/expression_tree.git
cd expression_tree
cargo run
```

O programa pede uma expressão, mostra a forma legível, a árvore e o resultado.

Também é possível passar a expressão direto pela entrada padrão:

```bash
echo "2 * (3 + 4) - 5" | cargo run
```

## Exemplos

### Precedência

```
Digite uma expressão: 2 * (3 + 4) - 5

Forma legível:
2 * (3 + 4) - 5

Árvore sintática:
└── -
    ├── *
    │   ├── 2
    │   └── +
    │       ├── 3
    │       └── 4
    └── 5

Resultado: 9
```

### Negação e parênteses redundantes

Parênteses desnecessários são removidos na forma legível; os necessários são mantidos.

```
Digite uma expressão: ((1 - (2 - 3))) * -4

Forma legível:
(1 - (2 - 3)) * -4

Árvore sintática:
└── *
    ├── -
    │   ├── 1
    │   └── -
    │       ├── 2
    │       └── 3
    └── - (Neg)
        └── 4

Resultado: -8
```

### Erros

```
Digite uma expressão: 10 / 0
...
Erro ao avaliar a expressão
```

```
Digite uma expressão: 2 +
Erro: Token inesperado: None
```

## Como funciona

### 1. Representação: a árvore

Cada expressão é um valor do enum `Expressao`. Números são folhas; operadores são nós que guardam suas subexpressões em `Box` (necessário porque o tipo é recursivo):

```rust
enum Expressao {
    Numero(i64),
    Negacao(Box<Expressao>),
    Adicao(Box<Expressao>, Box<Expressao>),
    Subtracao(Box<Expressao>, Box<Expressao>),
    Multiplicacao(Box<Expressao>, Box<Expressao>),
    Divisao(Box<Expressao>, Box<Expressao>),
    Resto(Box<Expressao>, Box<Expressao>),
}
```

### 2. Parser: descida recursiva

A gramática tem um nível por grau de precedência. Cada nível vira uma função, que chama a do nível de precedência mais alta:

```
expr     → add_sub
add_sub  → mul_div  (('+' | '-') mul_div)*
mul_div  → unario   (('*' | '/' | '%') unario)*
unario   → '-' unario | primario
primario → número | '(' expr ')'
```

Como as funções de nível mais baixo são chamadas primeiro, `*` "gruda" mais forte que `+`. O laço em `add_sub` e `mul_div` constrói a árvore para a esquerda, o que garante que `7 - 2 - 1` seja lido como `(7 - 2) - 1`.

### 3. Avaliação

`avaliar` percorre a árvore recursivamente e devolve `Option<i64>`. Todas as operações usam `checked_add`, `checked_sub`, `checked_mul`, `checked_div`, `checked_rem` e `checked_neg`, que retornam `None` em caso de overflow ou divisão por zero; o operador `?` propaga esse `None` até o topo.

### 4. Impressão

- `imprimir` reconstrói o texto da expressão e só coloca parênteses quando a precedência do filho é menor que a do pai, ou no lado direito de operadores não associativos (`-`, `/`, `%`), como em `7 - (2 - 1)`.
- `imprimir_arvore` desenha a árvore com os caracteres `├──`, `└──` e `│`.

## Estrutura do projeto

```
expression_tree/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── main.rs      # programa principal (enum, parser, avaliação e impressão)
    └── exemplo.rs   # rascunho antigo, não é compilado pelo Cargo
```

## Limitações

- Apenas números inteiros (`i64`); não há suporte a decimais
- Sem variáveis nem funções (como `x` ou `sqrt`)
- A divisão é inteira (`7 / 2 = 3`)
- Lê uma expressão por execução

## Ideias para evoluir

- Suporte a números decimais (`f64`)
- Variáveis e atribuição (`x = 3`, `x * 2`)
- Potenciação (`^`), com associatividade à direita
- Modo interativo (REPL) para várias expressões seguidas
- Mensagens de erro indicando a posição do problema na entrada
- Testes automatizados com `cargo test`
