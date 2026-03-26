#include "Wheels.h"



Wheels w;
bool isInMove = false;
volatile char cmd;

void setup() {
  // put your setup code here, to run once:
  w.attach(4,5,6,7,10,11);
  
  Serial.begin(9600);
  Serial.println("Forward: WAD");
  Serial.println("Back: ZXC");
  Serial.println("Stop: S");

  delay(5001);
  for (int i = 0; i < 100; i++) {
    w.goForward(100);
    w.stop();
    delay(50);
    w.goBack(100);
    w.stop();
    delay(50);   
  }    
}

void loop() {
  while(Serial.available())
  {
    w.goForward(25);
    w.goBack(25);
    w.stop();
    cmd = Serial.read();
    switch(cmd)
    {
      case 'w': w.forward(); break;
      case 'x': w.back(); break;
      case 'a': w.forwardLeft(); break;
      case 'd': w.forwardRight(); break;
      case 'z': w.backLeft(); break;
      case 'c': w.backRight(); break;
      case 's': w.stop(); break;
      case '1': w.setSpeedLeft(75); break;
      case '2': w.setSpeedLeft(200); break;
      case '9': w.setSpeedRight(75); break;
      case '0': w.setSpeedRight(200); break;
      case '5': w.setSpeed(100); break;
      case 'q': forwardBack(); break;
      case 'p': isInMove = !isInMove;
    }
  }
}

void forwardBack() {
  w.setSpeedRight(200);
  w.setSpeedLeft(200);
  while (!isInMove) {
    w.forward();
    delay(500);
    w.back();
    delay(200);
    w.back();
    delay(500);
    w.forward();
    delay(200);
  }
}
