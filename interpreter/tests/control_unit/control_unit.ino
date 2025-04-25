#define P_IO PORTF
#define D_IO DDRF
#define I_IO PINF

#define P_ROM PORTK
#define D_ROM DDRK

#define P_RAM PORTA
#define D_RAM DDRA
#define I_RAM PINA

#define INPUT_AWAIT 37
#define PI_OR_OR 36
#define OUTPUT_CONFIRMED 35
#define STARTUP 34
#define STATE_JUMPING 33
#define NOT_RESET 32
#define NOT_CLOCK 31

const bool debugging = false;
const char *characters = "><+-.,[]";
const int DELAY = 100;

char userwait() {
  while (!Serial.available()) {}
  return Serial.read();
}

void delaywait() {
  if (debugging) {
    userwait();
  } else if (DELAY) {
    delay(DELAY);
  }
}

void pbin(char n) {
  for (int j = 7; j >= 0; j--) {
    Serial.print((n >> j) & 0b1);
  }
}

void logInterrupt2() {
  Serial.print("Probe 2 now ");
  Serial.println(digitalRead(2) ? "HIGH" : "LOW");
}

void logInterrupt3() {
  Serial.print("Probe 3 now ");
  Serial.println(digitalRead(3) ? "HIGH" : "LOW");
}

void setup() {
  // put your setup code here, to run once:
  /*attachInterrupt(digitalPinToInterrupt(2), logInterrupt2, CHANGE);
  attachInterrupt(digitalPinToInterrupt(3), logInterrupt3, CHANGE);*/

  Serial.begin(9600);
  Serial.println();
  Serial.println();
  
  D_ROM = 0b11111111;
  D_IO = 0b00000000;
  D_RAM = 0b00000000;
  // All OUTPUT except INPUT_AWAIT and PI_OR_OR
  DDRC = 0b01111100;

  P_ROM = 0b0;
  P_IO = 0b0;
  P_RAM = 0b0;
  // STARTUP
  PORTC = 0b00001000;
  delaywait();
  digitalWrite(NOT_RESET, HIGH);

  // comparator tests
  for (size_t i = 0; i < strlen(characters) + 1; i++) {
    P_ROM = characters[i];
    Serial.print("LED for character '");
    Serial.print(characters[i]);
    Serial.print("' (");
    pbin(characters[i]);
    Serial.println(") should now be on");
    userwait();
  }

  digitalWrite(STARTUP, LOW);
  Serial.println("startup LED should be on");
  userwait();

  digitalWrite(STARTUP, HIGH);
  Serial.println("startup LED should be off, finished LED should be on");
  userwait();

  P_ROM = '<';
  Serial.println("running LED should be on");
  userwait();

  digitalWrite(STATE_JUMPING, HIGH);
  Serial.println("jump LED should be on");
  userwait();
  digitalWrite(STATE_JUMPING, LOW);

  // input
  Serial.println("Testing input");
  P_ROM = ',';
  digitalWrite(NOT_CLOCK, HIGH);
  Serial.println("input LED should be on");
  userwait();

  Serial.println("Cycling clock 5 times to test if input will still be requested");
  for (int i = 0; i < 10; i++) {
    // ending on high
    digitalWrite(NOT_CLOCK, i % 2);
    delaywait();
  }

  Serial.println("input LED should still be on");
  userwait();

  if (!digitalRead(INPUT_AWAIT)) {
    Serial.println("INPUT_AWAIT is not on. Test failed.");
    return;
  }
  Serial.println("INPUT_AWAIT is on. Test passed.");

  Serial.println("Inputting 0xff and giving a pulse on PUSH_INPUT");
  D_IO = 0b11111111;
  P_IO = 0b11111111;
  pinMode(PI_OR_OR, OUTPUT);
  digitalWrite(PI_OR_OR, LOW);
  delaywait();
  digitalWrite(PI_OR_OR, HIGH);
  delaywait();
  digitalWrite(PI_OR_OR, LOW);
  delaywait();

  Serial.println("Testing if input was successfull");
  if (I_RAM != P_IO) {
    Serial.print("expected RAM_IN (");
    pbin(I_RAM);
    Serial.print(") to equal IO_OUT (");
    pbin(P_IO);
    Serial.println("). Test failed.");
    return;
  }
  Serial.println("RAM_IN equals IO_OUT. Test passed.");

  Serial.println("Cycling clock to test if INPUT_AWAIT will go off");
  digitalWrite(NOT_CLOCK, LOW);
  delaywait();
  digitalWrite(NOT_CLOCK, HIGH);
  delaywait();
  digitalWrite(NOT_CLOCK, LOW);
  delaywait();

  if (digitalRead(INPUT_AWAIT)) {
    Serial.println("INPUT_AWAIT is still on. Test failed.");
    return;
  }
  Serial.println("INPUT_AWAIT is off. Test passed.");

  // output
  pinMode(PI_OR_OR, INPUT);
  Serial.println("testing output");
  P_ROM = '.';
  D_IO = 0b00000000;
  D_RAM = 0b11111111;
  P_RAM = 0b10101010;
  userwait();
  digitalWrite(NOT_CLOCK, HIGH);
  Serial.println("output LED should be on");
  userwait();
  
  Serial.println("Cycling clock 5 times to test if output is still ready");
  for (int i = 0; i < 10; i++) {
    // ending on high
    digitalWrite(NOT_CLOCK, i % 2);
    delaywait();
  }

  Serial.println("output LED should still be on");
  userwait();

  if (!digitalRead(PI_OR_OR)) {
    Serial.println("OUTPUT_READY is not on. Test failed.");
    return;
  }
  Serial.println("OUTPUT_READY is on. Test passed.");

  Serial.println("Testing if data is there");
  if (P_RAM != I_IO) {
    Serial.print("expected RAM_OUT (");
    pbin(P_RAM);
    Serial.print(") to equal IO_IN (");
    pbin(I_IO);
    Serial.println("). Test failed.");
    return;
  }
  Serial.println("RAM_OUT equals IO_IN. Test passed.");

  Serial.println("Giving a pulse on OUTPUT_CONFIRMED");
  pinMode(OUTPUT_CONFIRMED, OUTPUT);
  digitalWrite(OUTPUT_CONFIRMED, LOW);
  delaywait();
  digitalWrite(OUTPUT_CONFIRMED, HIGH);
  delaywait();
  digitalWrite(OUTPUT_CONFIRMED, LOW);
  delaywait();

  Serial.println("Cycling clock to test if OUTPUT_READY will go off");
  digitalWrite(NOT_CLOCK, LOW);
  delaywait();
  digitalWrite(NOT_CLOCK, HIGH);
  delaywait();
  digitalWrite(NOT_CLOCK, LOW);
  delaywait();

  if (digitalRead(PI_OR_OR)) {
    Serial.println("OUTPUT_READY is still on. Test failed.");
    return;
  }
  Serial.println("OUTPUT_READY is off. Test passed.");
}

void loop() {
  // put your main code here, to run repeatedly:

}
