#define WRITE_ENABLE 2
#define STARTUP1 3
#define CONTROL_ENABLE 4
#define CONTROL_CLOCK 5

#define FLAG_WRITE_ENABLE 0b1
#define FLAG_STARTUP1 0b10
#define FLAG_CONTROL_ENABLE 0b100

/**
  CLOCK_JK.1Q connected to oscilloscope
*/

const char *TEST_LABELS[] = {
  "write enable on -> there should be no clock (enter to continue)",
  "control enable on -> press 'c' to cycle clock and enter to continue testing",
  "startup1 off -> clock should be at 1MHz (enter to continue)",
  "startup1 on -> you should be able to switch between fast and slow clock (testing done)"
};

const char REGISTER_VALUES[] = {
  FLAG_WRITE_ENABLE | FLAG_STARTUP1,
  FLAG_CONTROL_ENABLE | FLAG_STARTUP1,
  0b0,
  FLAG_STARTUP1
};

const int TEST_LENGTH = 4;

bool control_clock = false;

void setup() {
  // put your setup code here, to run once:
  pinMode(WRITE_ENABLE, OUTPUT);
  pinMode(STARTUP1, OUTPUT);
  pinMode(CONTROL_ENABLE, OUTPUT);
  pinMode(CONTROL_CLOCK, OUTPUT);

  Serial.begin(9600);
  Serial.println();

  for (int i = 0; i < TEST_LENGTH; i++) {
    Serial.print("WRITE_ENABLE: ");
    Serial.println(REGISTER_VALUES[i] & FLAG_WRITE_ENABLE ? "true" : "false");
    Serial.print("STARTUP1: ");
    Serial.println(REGISTER_VALUES[i] & FLAG_STARTUP1 ? "true" : "false");
    Serial.print("CONTROL_ENABLE: ");
    Serial.println(REGISTER_VALUES[i] & FLAG_CONTROL_ENABLE ? "true" : "false");

    digitalWrite(WRITE_ENABLE, REGISTER_VALUES[i] & FLAG_WRITE_ENABLE);
    digitalWrite(STARTUP1, REGISTER_VALUES[i] & FLAG_STARTUP1);
    digitalWrite(CONTROL_ENABLE, REGISTER_VALUES[i] & FLAG_CONTROL_ENABLE);

    Serial.println(TEST_LABELS[i]);

    bool enter = false;
    while (!enter) {
      while (!Serial.available()) {}

      switch (Serial.read()) {
        case '\n':
          enter = true;
          break;
        case 'c':
          control_clock = !control_clock;
          digitalWrite(CONTROL_CLOCK, control_clock);
          break;
        default:
          Serial.print("unknown command.\n");
          Serial.println(TEST_LABELS[i]);
          break;
      }
    }
  }
}

void loop() {
  // put your main code here, to run repeatedly:

}
