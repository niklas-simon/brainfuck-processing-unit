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
-	I/O durch 10-pin-Verbindung
    - 8-bit I/O
        - MSB 0: ASCII, 1: reserviert (evtl. 7 Pins)
    - 1 Bit Richtung (1: Input, 0: Output)
    - 1 Bit Clock
- Clock durch Potentiometer o.ä. einstellbar
- Programmierung EEPROM durch Raspberry PI (welcher auch Digital Twin steuert)
### I/O Management & Digital Twin
- Raspberry PI
- Verwaltet 18-pin-Verbindung des Interpreters
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
- [ ] Aufbau aller von Logisim bereitgestellten Komponenten auf Breadboard
    - [ ] RAM (SRAM)
    - [ ] Zähler
    - [ ] ROM (EEPROM)
    - [ ] Etc.
- [ ] Aufbau Interpreter auf Breadboards
- [ ] Entwicklung Interpreter PCB
- [ ] Entwicklung I/O-Manager
- [ ] Entwicklung Digital Twin
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
    1. Direction (1: Input, 0: Output)
    2. Clock
    3. Ready for Output
    4. I/O 0
    5. I/O 1
    6. I/O 2
    7. I/O 3
    8. I/O 4
    9. I/O 5
    10. I/O 6
    11. I/O 7
    12. Ground