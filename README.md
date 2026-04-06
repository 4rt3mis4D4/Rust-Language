# 🦀 Rust Exercises Roadmap

Repositório dedicado à resolução de **100 exercícios em Rust**, estruturados para levar do nível básico ao domínio dos principais conceitos da linguagem, com foco especial em **ownership**, **borrowing** e **segurança de memória**.

---

## 🚀 Objetivo

Desenvolver domínio sólido em Rust através de uma trilha prática que estimula:

- Pensamento lógico e resolução de problemas
- Compreensão profunda da sintaxe
- Gerenciamento seguro de memória
- Escrita de código idiomático e performático

---

## 🧠 Estrutura do Roadmap

Os exercícios estão organizados em **6 fases progressivas**, combinando teoria + prática:

---

### 📌 Fase 1: Tipos de Dados e Strings (1–20)
Fundamentos de como Rust representa dados:

- Tipos escalares (`i32`, `u32`, `f64`, `char`)
- Tuplas e arrays
- `String` vs `&str`
- Conversões de tipos
- Noções básicas de memória

📌 Aplicações práticas:
- Operações matemáticas com tuplas
- Conversor de temperatura
- Cálculo de distância entre pontos
- Manipulação e análise de strings
- Simulação de estruturas simples (pilha)

---

### 🔁 Fase 2: Variáveis, Mutabilidade e Shadowing (21–40)
Controle de estado e ciclo de vida de variáveis:

- Imutabilidade por padrão
- Uso de `mut`
- Shadowing
- Constantes (`const`)
- Escopo de variáveis

📌 Aplicações práticas:
- Sistemas de pontuação
- Cálculo de área e juros
- Transformações com shadowing
- Regras de escopo na prática

---

### 🖥️ Fase 3: Entrada e Saída (I/O) (41–60)
Interação com o usuário e formatação:

- `println!`, `print!`, `format!`
- Leitura com `stdin`
- Parsing de dados
- Formatação avançada
- Tratamento básico de erros

📌 Projetos:
- Calculadora interativa
- Recibo formatado
- Contador de vogais
- Tabuada
- Simulação de progresso

---

### 🔄 Fase 4: Condições e Repetições (61–80)
Controle de fluxo e lógica:

- `if`, `else`
- `match`
- `loop`, `while`, `for`
- `break` e `continue`
- Pattern matching

📌 Desafios clássicos:
- FizzBuzz
- Fibonacci
- Jogo de adivinhação
- Menu interativo
- Pirâmide de asteriscos

---

### 🔐 Fase 5: Funções, Ownership e Borrowing (81–95)
O diferencial do Rust:

- Definição de funções
- Ownership (movimentação de dados)
- Borrowing (`&` e `&mut`)
- Regras do borrow checker
- Slices e lifetimes básicos

📌 Exercícios:
- Manipulação segura de `String` e `Vec`
- Algoritmos (Bubble Sort)
- Palíndromos
- Busca sem perda de ownership

---

### 💾 Fase 6: Arquivos e Projeto Final (96–100)
Persistência de dados:

- Leitura e escrita de arquivos
- Append com `OpenOptions`
- Processamento de dados

📌 Projeto final:
- ✅ Sistema de **To-Do List** com persistência em arquivo

---

## 🛠️ Tecnologias

- Rust (stable)
- Cargo (gerenciador de pacotes padrão)
