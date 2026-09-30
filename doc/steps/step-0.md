# FASE 0

Neste momento, o código apenas faz o toggle do GPIO8 e incrementa uma variável global.

À primeira vista, isso pode parecer algo simples e até sem muita utilidade prática. De fato, o comportamento observado é apenas um LED piscando e um contador sendo incrementado.

Porém, essa fase é importante porque estabelece a base sobre a qual o restante do RTOS será construído.

Sobre o código

O código possui três partes principais:

Configuração do GPIO.
Configuração do timer e da ISR.
Tratamento de pânico (panic_handler).

A parte mais importante é a ISR (timer_isr).

Quando o timer gera uma interrupção, a função timer_isr() é executada. Nela são realizadas duas ações:

Toggle do GPIO8.
Incremento da variável global TICK_COUNT.
O que isso significa?

No final da função main() existe apenas um loop infinito:

```rust
loop {
    let current = TICK_COUNT.load(Ordering::Relaxed);
    info!("{}", current);
}
```

Analisando apenas esse trecho, não existe nenhuma instrução que altere TICK_COUNT ou faça o LED piscar.

Então como isso acontece?

A resposta está na interrupção do timer.

A cada período configurado, o hardware interrompe temporariamente a execução normal da CPU e transfere o controle para a função timer_isr().

Dessa forma, a ISR pode executar código independentemente do que a função main() esteja fazendo.

Relação com um Scheduler

Um scheduler precisa de alguma forma de medir o tempo e tomar decisões periodicamente.

Em sistemas operacionais e RTOSs, isso geralmente é feito através de um "tick de sistema".

O TICK_COUNT criado nesta fase representa exatamente essa ideia.

Hoje ele apenas é incrementado:

TICK_COUNT += 1


Mas em fases futuras ele poderá ser utilizado para:

Controlar atrasos (delay).
Despertar tarefas bloqueadas.
Controlar timeouts.
Disparar eventos periódicos.
Determinar quando uma tarefa deve ceder espaço para outra.

Em outras palavras, a interrupção do timer é o mecanismo que permitirá ao futuro scheduler executar código de forma periódica e independente do fluxo principal da aplicação.

Sem esse tick periódico, seria muito mais difícil implementar um escalonador semelhante aos encontrados em FreeRTOS, Zephyr ou outros RTOSs.

