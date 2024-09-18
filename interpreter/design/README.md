# Interpreter
## Abkürzungen
- `D` Data
- `DP` Data Pointer
- `IR` Increment Register
- `DR` Decrement Register
- `OUT` Ouput Register
- `OE` Output Enable
- `OR` Ready to Receive Output
- `IN` Input Register
- `IE` Input Enable
- `JP` Jump Pointer
## States
- `ENABLED` Interpreter ist aktiv
- `STARTUP` Programm beginnt auf erster fallender Flanke. Falls davor eine steigende ist, darf diese nicht abgehandelt werden
- `JUMPING` Interpreter springt auf nächste ']'
- `INPUT` Interpreter wartet auf Input
- `OUTPUT` Interpreter wartet auf Berechtigung, Output zu senden
## Ablauf
- steigende Flanke
    - `ENABLED`
        - `PC++`
        - `IR = D + 1`
        - `DR = D - 1`
        - `OE == TRUE` `OE = FALSE`
    - `STARTUP` State `ENABLED`
    - `JUMPING`
        - `*PC == ']'` State `ENABLED`
        - `*PC != ']'` `PC++`
    - `INPUT`
        - `IE == TRUE` State `ENABLED`
        - `IE == FALSE` NOP
    - `OUTPUT`
        - `OR == TRUE` State `ENABLED`
        - `OR == FALSE` NOP
- fallende Flanke
    - `ENABLED`
        - `>` `DP++`
        - `<` `DP--`
        - `+` `D = IR`
        - `-` `D = DR`
        - `.`
            - `OUT = D`
            - `OR == FALSE` State `OUTPUT`
            - `OR == TRUE` `OE = TRUE`
        - `,` 
            - `IE == TRUE`
                - `D = IN`
                - `IE = FALSE`
            - `IE == FALSE` State `INPUT`
        - `[`
            - `D == 0`
                - `JP++`
                - State `JUMPING`
            - `D != 0`
                - `*JP = PC`
                - `JP++`
        - `]`
            - `D == 0` `JP--`
            - `D != 0`
                - `PC = *(JP - 1)`
                - `JP--`
    - `STARTUP` NOP
    - `JUMPING` NOP
    - `INPUT` NOP
    - `OUTPUT` NOP