#include <Arduino.h>
#include <Wire.h>
#include <LiquidCrystal_I2C.h>
#include "Wheels.h"

Wheels w;
LiquidCrystal_I2C lcd(0x27, 16, 2); 

unsigned long lastLcdUpdate = 0;
int animFrame = 0;

void setup() {
    w.attach(4,5,6,7,10,11);
    
    Serial.begin(9600);
    
    lcd.init();
    lcd.backlight();
    lcd.clear();
    lcd.print("Autko gotowe");
    delay(1000);
    lcd.clear();

    for (int i = 100; i > 0; i = i - 10) {
      w.goForward(i);
      while (w.) {
        w.update();
        if (millis() - lastLcdUpdate > 200) {
          lastLcdUpdate = millis();
          updateLCD();
        }
      }
    }
}

void loop() {
   w.update();

    if (Serial.available()) {
        char cmd = Serial.read(); 
        switch(cmd) {
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
          case 'u': w.goForward(100); break;
          case 'p': w.goBack(100); break;
        }
    }

    if (millis() - lastLcdUpdate > 200) {
        lastLcdUpdate = millis();
        updateLCD();
    }
}

void updateLCD() {
    lcd.setCursor(0, 0);
    int dist = w.getRemainingDistance();
    if (dist > 0) {
        lcd.print("Do celu: ");
        if (dist < 10) lcd.print(" "); 
        lcd.print(dist);
        lcd.print(" cm  ");
    } else {
        lcd.print("Cel osiagniety  ");
    }

    lcd.setCursor(0, 1);
    
    lcd.print("L:");
    int sL = w.getSpeedLeft();
    if (sL >= 0 && sL < 100) lcd.print(" "); // wyrównanie
    lcd.print(sL);
    lcd.print(" ");

    // Animacja na środku
    int dir = w.getDirection();
    if (dir == 1) {
        lcd.print(" >");
    } else if (dir == -1) {
      lcd.print(" <");
    } else {
        lcd.print("-");
    }
    lcd.print(" ");

    // Prędkość prawa
    lcd.print("P:");
    int sR = w.getSpeedRight();
    if (sR >= 0 && sR < 100) lcd.print(" "); // wyrównanie
    lcd.print(sR);
    lcd.print("  "); 
}

