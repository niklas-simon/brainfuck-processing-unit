// C++ code
//

const unsigned int ENCODING[] = {0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F, 0x77, 0x7C, 0x39, 0x5E, 0x79, 0x71};

unsigned int show_digit = 0;
unsigned int read_digit = 1;

void setup()
{
  // 0-7: output
  DDRD = 0xff;
  // 8-13: output
  DDRB = 0b111111;
  // A0-A5: output
  DDRC = 0b111111;
}

void loop()
{
  // read number from A0-A3
  unsigned int number = PINC & 0b1111;
  // set A4-A5 to most significant bits of next-to-read display
  PORTC = (PORTC & 0b1111) | ((read_digit & 0b1100) << 2);
  // shut off displays by showing display #15 which doesn't exist
  // set 12-13 to least significant bits of next-to-read display
  PORTB = 0b1111 | ((read_digit & 0b11) << 4);
  // set display to show number
  PORTD = ENCODING[number];
  // show correct display
  PORTB = (PORTB & 0b11110000) | (show_digit & 0b1111);
  // increment show_digit for next cycle
  show_digit = (show_digit + 1) % 10;
  // increment read_digit for next cycle
  read_digit = (read_digit + 1) % 10;
  
  delay(1000);
}