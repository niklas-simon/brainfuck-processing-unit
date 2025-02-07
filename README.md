# Hardware Brainfuck Interpreter
Niklas Pein (G220093PI), Bernhard Lindner (G220360PI)
## Anforderungen
### Interpreter
- Ausführung von Brainfuck-Programmen nach Standard Brainfuck Spezifikation
- Interpreter besteht lediglich aus nicht-programmierbaren Hardwarekomponenten
    - Logik-Gatter
    - SRAM
    - Zähler
    - Register
    - Etc.
-	I/O durch 12-pin-Verbindung
    1. 8-bit I/O
        - MSB 0: ASCII, 1: reserviert
        - Richtung durch `2.` vorgegeben
    2. Warten auf Input
        - 1: Input kann durch Pulse auf `3.` übergeben werden
        - 0: Sollte `3.` `HIGH` sein, kann Output gelesen werden
    3. Push Input / Output bereit
        - Funktion wird durch `2.` bestimmt
    4. Output Confirmed
        - Durch Pulse kann das Lesen des Outputs bestätigt werden und der Interpreter wird fortgesetzt
- Clock durch Potentiometer o.ä. einstellbar
- Programmierung EEPROM durch Raspberry PI (welcher auch Digital Twin steuert)
### I/O Management & Digital Twin
- Raspberry PI
- Verwaltet Verbindungen zum Interpreter
    - 12 Pin I/O
    - 4 Pin ROM-Programmierung
    - 4 Pin Steuerport
- Zeigt Output des Interpreters
- Gibt Tastatureingaben als Input an Interpreter
- Zeigt Interpreter-Schaltung in Simulation
- Zeigt Zellenbelegung und Position im Programm
- Synchronisation der Clock des Interpreters mit Twin
- Einlesen des ROMs und I/O zur Ausführung parallel zum Twin
### Darstellung
- Soll zu jeder Zeit interessant und beschäftigt wirken
- 7-Segment-Anzeigen zeigen Adresse und Wert von ROM und beiden RAMs
- LEDs zeigen
    - Warten auf Input
    - Senden von Output
    - Etc.
## Entwicklungsschritte
- [x] Konzepterstellung Interpreter in Logisim
- [x] Entwicklung Schaltkreis-Diagramm zur Aufstellung einer Teilliste
- [ ] Aufbau Segmente aus Schaltkreis-Diagramm auf Breadboard
    - [x] Display 
    - [ ] ROM (EEPROM) (8h)
    - [ ] RAM (SRAM) (8h)
    - [ ] Jump Unit (8h)
    - [ ] Control Unit (8h)
    - [ ] Zusammenfügen (16h)
- [ ] Entwicklung Interpreter PCB (8h)
- [ ] Entwicklung I/O-Manager
    - [x] Oberfläche
    - [ ] Hardware-Integration (16h)
- [x] Entwicklung Digital Twin

## Spezifikationen
### Programmierung
- 4 Pin Port
    1. Write Enable
    2. Clock
    3. Data
    4. Ground
- Ablauf
    1. Write Enable -> HIGH
    2. Bit in Data laden
    3. Clock -> HIGH -> LOW
    4. für jedes Bit zurück zu 2.
    5. Write Enable -> LOW
### Kontrolle
- 4 Pin Port
    1. Control Enable
    2. Clock
    3. Reset
    4. Ground
### I/O
- 12 Pin Port
    1. Input Await
    2. Push Input / Output Ready
    3. Output Confirmed
    4. I/O 0
    5. I/O 1
    6. I/O 2
    7. I/O 3
    8. I/O 4
    9. I/O 5
    10. I/O 6
    11. I/O 7
    12. Ground
- Ablauf
    - Input
        1. **Interpreter**: Input Await -> HIGH
        2. **External**: I/O-Pins setzen
        3. **External**: Push Input -> HIGH -> LOW
    - Output
        1. **Interpreter**: I/O-Pins setzen
        1. **Interpreter**: Output Ready -> HIGH
        2. **External**: I/O-Pins verarbeiten
        3. **External**: Output Confirmed -> HIGH -> LOW
