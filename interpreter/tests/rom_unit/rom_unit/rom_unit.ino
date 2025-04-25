/* Connections:
2: WRITE_ENABLE
3: WRITE_CLOCK
4: WRITE_DATA
5: PC_ENABLE
6: PC_CLOCK
7: !RESET

8-11: ROM_IO 0-3
A0-A3: ROM_IO 4-7
*/

const bool debug = false;

const unsigned char *content = "<>+-.,[]";

int wait_time = 0;

void setup() {
  Serial.begin(9600);

  DDRD = 0b11111100;
  DDRB = 0b0;
  DDRC = 0b0;

  // disable reset and enable writing
  PORTD |= 0b10000100;
  if (debug) {
    Serial.println();
    Serial.print(PORTD, BIN);
    Serial.println();
  }
  if (wait_time) delay(wait_time);

  // write
  for (unsigned int i = 0; i <= strlen(content); i++) {
    Serial.print("writing... ");
    for (int j = 7; j >= 0; j--) {
      unsigned int bit = (content[i] >> j) & 0b1;
      Serial.print(~bit & 0b1);

      // set WRITE_DATA to current bit
      PORTD = (PORTD & 0b11101111) | (bit << 4);
      if (wait_time) delay(wait_time);

      // set WRITE_CLOCK to 1
      PORTD |= 0b1000;
      if (wait_time) delay(wait_time);

      // set WRITE_CLOCK to 0
      PORTD &= 0b11110111;
      if (wait_time) delay(wait_time);
    }

    unsigned char rom = (PINB & 0b1111) | ((PINC & 0b1111) << 4);
    Serial.print("\nBUF_IO: ");
    Serial.print(rom, BIN);
    Serial.println();

    bool isGood = (rom & 0b11111111) == ((~content[i]) & 0b11111111);
    Serial.print("test: ");
    Serial.println(isGood ? "passed" : "failed");
      
    if (debug) {
      while (!Serial.available()) {}
      Serial.read();
    }
  }

  // disable writing and do reset
  PORTD &= 0b01111011;
  if (wait_time) delay(wait_time);
  PORTD |= 0b10000000;
  if (wait_time) delay(wait_time);

  // enable reading
  PORTD |= 0b100000;
  if (wait_time) delay(wait_time);

  // read and compare
  for (unsigned int i = 0; i <= strlen(content); i++) {
    unsigned char rom = (PINB & 0b1111) | ((PINC & 0b1111) << 4);
    Serial.print("\nROM_IO: ");
    Serial.print(rom, BIN);
    Serial.println();

    bool isGood = (rom & 0b11111111) == ((~content[i]) & 0b11111111);
    Serial.print("test: ");
    Serial.println(isGood ? "passed" : "failed");

    // let clock tick
    if (debug) {
      while (!Serial.available()) {}
      Serial.read();
    }
    PORTD |= 0b1000000;
    if (wait_time) delay(wait_time);
    if (debug) {
      while (!Serial.available()) {}
      Serial.read();
    }
    PORTD &= 0b10111111;
    if (wait_time) delay(wait_time);
  }

  // disable reading
  PORTD &= 0b11011111;
  if (wait_time) delay(wait_time);
}

void loop() {
  // put your main code here, to run repeatedly:
}
