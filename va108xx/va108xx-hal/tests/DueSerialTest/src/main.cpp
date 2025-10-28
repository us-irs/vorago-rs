#include <Arduino.h>

enum SendModes {
  ECHO,
  ONLY_WRITE,
  ONLY_READ,
  WRITE_READ
};

enum StringModes {
  FIXED,
  VARIABLE
};

// Configure the test application here
SendModes SEND_MODE = SendModes::WRITE_READ;
StringModes STRING_MODE = StringModes::VARIABLE;
uint8_t STRING_IDX = 0;

String STRINGS[4] = {
  "$Hi\n",
  "$Hello\n",
  "$Hello World\n",
  "$Hello and Merry Christmas to all of you!\n"
};

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  Serial.println("Starting Arduino Serial Test script..");
  Serial1.begin(115200);
  if(STRING_MODE == StringModes::VARIABLE) {
    STRING_IDX = 0;
  }
  pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
  static byte ICOMING_BYTE = 0;
  static uint32_t GLOBAL_IDX = 0;
  digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
  // put your main code here, to run repeatedly:
  // send data only when you receive data:
  if (SEND_MODE == SendModes::ONLY_WRITE or SEND_MODE == SendModes::WRITE_READ) {
    Serial.println("Sending string..");
    Serial1.write(STRINGS[STRING_IDX].c_str());
    if(STRING_MODE == StringModes::VARIABLE) {
      STRING_IDX += 1;
      if(STRING_IDX > 3) {
        STRING_IDX = 0;
      }
    }
  }
  if(
    SEND_MODE == SendModes::WRITE_READ or
    SEND_MODE == SendModes::ONLY_READ or
    SEND_MODE == SendModes::ECHO
  ) {
    if (Serial1.available() > 0) {
      // read the incoming byte:
      String readString = Serial1.readStringUntil('\n');

      Serial.print(GLOBAL_IDX);
      Serial.print(" - ");
      GLOBAL_IDX++;
      // say what you got:
      Serial.print("I received: ");
      Serial.println(readString);
      if(SEND_MODE == SendModes::ECHO) {
        delay(200);
        Serial.println("Sending back echo message");
        String sendBack = readString + '\n';
        Serial1.write(sendBack.c_str());
      }
    }
  }
  delay(3000);
}
