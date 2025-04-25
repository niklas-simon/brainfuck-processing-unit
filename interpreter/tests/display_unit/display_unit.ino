#define P_PC0 PORTF
#define D_PC0 DDRF
#define P_PC1 PORTK
#define D_PC1 DDRK

#define P_ADDR0 PORTA
#define D_ADDR0 DDRA
#define P_ADDR1 PORTC
#define D_ADDR1 DDRC

#define P_DATA PORTL
#define D_DATA DDRL

#define DELAY 100
#define DEBUG false

void wait() {
  if (DEBUG) {
    while (!Serial.available()) {}
    Serial.read();
  } else {
    delay(DELAY);
  }
}

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  Serial.println();
  Serial.println();

  D_PC0 = 0xff;
  D_PC1 = 0xff;
  D_ADDR0 = 0xff;
  D_ADDR1 = 0xff;
  D_DATA = 0xff;

  P_PC0 = 0;
  P_PC1 = 0;
  P_ADDR0 = 0;
  P_ADDR1 = 0;
  P_DATA = 0;

  Serial.println("testing pc");
  wait();

  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 0x10; j++) {
      int n = j << (i * 4);
      P_PC0 = n & 0xff;
      P_PC1 = (n >> 8) & 0xff;
      wait();
    }
  }

  Serial.println("testing addr");
  wait();

  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 0x10; j++) {
      int n = j << (i * 4);
      P_ADDR0 = n & 0xff;
      P_ADDR1 = (n >> 8) & 0xff;
      wait();
    }
  }

  Serial.println("testing data");
  wait();

  for (int i = 0; i < 0x100; i++) {
    P_DATA = i & 0xff;
      wait();
  }
}

void loop() {
  // put your main code here, to run repeatedly:

}
