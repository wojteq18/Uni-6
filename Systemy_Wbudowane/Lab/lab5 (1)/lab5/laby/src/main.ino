// #include <Servo.h>

// // piny dla sonaru (HC-SR04)
// #define TRIG A4
// #define ECHO A5

// // pin kontroli serwo (musi być PWM)
// #define SERVO 9

// Servo serwo;

// void setup() {
//   pinMode(TRIG, OUTPUT);    // TRIG startuje sonar
//   pinMode(ECHO, INPUT);     // ECHO odbiera powracający impuls

//   Serial.begin(9600);

//   serwo.attach(SERVO);

// /* rozejrzyj się w zakresie od 0 stopni (patrz na jedną burtę)
//  *  do 180 stopni (patrz na prawą burtę). Wydrukuj na konsoli
//  *  kierunek patrzenia i najbliżej widziany obiekt (pojedynczy pomiar)
//  */
//   for(byte angle = 0; angle < 180; angle+= 20) {
//     lookAndTellDistance(angle);
//     delay(500);
//   }
  
// /* patrz przed siebie */
//   serwo.write(90);

// }

// void loop() { /* nic nie rób */ }

// void lookAndTellDistance(byte angle) {
  
//   unsigned long tot;      // czas powrotu (time-of-travel)
//   unsigned int distance;

//   Serial.print("Patrzę w kącie ");
//   Serial.print(angle);
//   serwo.write(angle);
  
// /* uruchamia sonar (puls 10 ms na `TRIGGER')
//  * oczekuje na powrotny sygnał i aktualizuje
//  */
//   digitalWrite(TRIG, HIGH);
//   delay(10);
//   digitalWrite(TRIG, LOW);
//   tot = pulseIn(ECHO, HIGH);

// /* prędkość dźwięku = 340m/s => 1 cm w 29 mikrosekund
//  * droga tam i z powrotem, zatem:
//  */
//   distance = tot/58;

//   Serial.print(": widzę coś w odległości ");
//   Serial.println(distance);
// }


#include <LiquidCrystal_I2C.h>
#include "../lib/Wheels/src/Wheels.h"
#include "../lib/Ticker/src/Ticker.h"
#include <PinChangeInterrupt.h>
#include <Servo.h>

// piny dla sonaru (HC-SR04) - uwaga, A4 i A5 to sprzętowe piny I2C (SDA, SCL),
// których używa ekran LCD! Należy przepiąć sonar na inne piny np. A2 i A3.
#define TRIG A2
#define ECHO A3

// pin kontroli serwo (musi być PWM)
#define SERVO 12
#define SONAR_CENTER_ANGLE 110
// If the servo is mounted mirrored, flip this to false.
constexpr bool SONAR_HIGHER_ANGLE_IS_RIGHT = false;
Servo serwo;

// encoder pins (left -> A0, right -> A1)
#define LEFT_ENCODER_PIN A0
#define RIGHT_ENCODER_PIN A1

byte LCDAddress = 0x27;
LiquidCrystal_I2C lcd(LCDAddress, 16, 2);
int sonarAngle = SONAR_CENTER_ANGLE;
int sonarStep = 30;

Wheels wheels;
bool moving = true;
int currentDistance = 0;

// encoder ISR wrappers
void leftEncoderISR() {
  wheels.incrementLeftEncoder();
}

void rightEncoderISR() {
  wheels.incrementRightEncoder();
}

// Deklaracje funkcji
void goLoop();
void sonarTickerAction();
int measureAtAngle(int angle);
bool chooseRightEscapeSide();
void decideAndAct(int distance);
void runMotionUntilDone(unsigned long timeoutMs);
void moveBackAndAvoidObstacle();

Ticker moveTicker(100, goLoop);
Ticker sonarTicker(150, sonarTickerAction);

void stopMove() {
  wheels.stop();
  moving = false;
}

void moveBackAndLeft() {
  wheels.back();
  delay(1000);
  wheels.turnLeftDeg(180); // lub weź z Wheels prostą funkcję na skręt
  delay(500);
  moving = true; // po manewrze wznów jazdę
}

void moveBackAndRight() {
  wheels.back();
  delay(1000);
  wheels.turnRightDeg(180);
  delay(500);
  moving = true;
}

void lcdUpdate(int distance, int angle) {
  // Pierwsza linia: Pozostały dystans
  lcd.setCursor(0,0);
  lcd.print("Dist:           ");
  lcd.setCursor(6,0);
  lcd.print(distance);
  lcd.print(" cm"); 

  // Druga linia: Kąt patrzenia
  lcd.setCursor(0,1);
  lcd.print("Angle:          ");
  lcd.setCursor(7,1);
  lcd.print(angle);
  lcd.print(" deg");
}

int sonarCheck() {
  // uruchamia sonar (puls 10 us na `TRIGGER')
  digitalWrite(TRIG, HIGH);
  delayMicroseconds(10);
  digitalWrite(TRIG, LOW);
  unsigned long tot = pulseIn(ECHO, HIGH, 30000); // 30ms timeout  
  
  if (tot == 0) {
    return 400; // Zwróć 400 zamiast zera, gdy nie ma odbicia
  }
  
  int distance = tot/58;
  return distance;
}

int measureAtAngle(int angle) {
  serwo.write(angle);
  delay(250); // give servo time to settle before reading
  return sonarCheck();
}

bool chooseRightEscapeSide() {
  const int leftAngles[] = {10, 50, 70};
  const int rightAngles[] = {130, 150, 170};

  int minLeft = 400;  // Zaczynamy od maksymalnej wartości (400cm to brak przeszkody)
  int minRight = 400; 

  // Szukamy NAJBLIŻSZEJ przeszkody po lewej stronie
  for (int angle : leftAngles) {
    int dist = measureAtAngle(angle);
    if (dist < minLeft) {
      minLeft = dist;
    }
  }

  // Szukamy NAJBLIŻSZEJ przeszkody po prawej stronie
  for (int angle : rightAngles) {
    int dist = measureAtAngle(angle);
    if (dist < minRight) {
      minRight = dist;
    }
  }

  Serial.print("LEWA min=");
  Serial.print(minLeft);
  Serial.print(" PRAWA min=");
  Serial.println(minRight);

  // Wybieramy stronę, gdzie najbliższa przeszkoda jest oddalona najdalej (jest bezpieczniej)
  return minRight >= minLeft;
}

void sonarTickerAction() {
  serwo.write(sonarAngle);
  
  // Pomiar odległości
  int distance = sonarCheck();
  currentDistance = distance; // zapisz globalnie dla goLoop
  
  Serial.print("Krzycze z tickera! Angle: ");
  Serial.print(sonarAngle);
  Serial.print(" Dist: ");
  Serial.println(distance);

  // Aktualizacja ekranu
  lcdUpdate(distance, sonarAngle); 
  
  if (distance < 30) {
      decideAndAct(distance);
  }

  // Wyliczenie kąta na następny tick (ruch "w tę i z powrotem" tzn. sweep)
  sonarAngle += sonarStep;
  if (sonarAngle >= 170) {
    sonarAngle = 170;
    sonarStep = -30;
  } else if (sonarAngle <= 50) {
    sonarAngle = 50;
    sonarStep = 30;
  }
}

void decideAndAct(int distance) {
  // jezeli przeszkoda jest blisko (np. <20cm) to zatrzymujemy sie i podejmujemy decyzje o omijaniu
  if (distance < 20) {
    stopMove();

    moveBackAndAvoidObstacle();

    // return sonar to front view after side checks
    serwo.write(SONAR_CENTER_ANGLE);
    sonarAngle = SONAR_CENTER_ANGLE;

    moving = true;
  }
}

void runMotionUntilDone(unsigned long timeoutMs) {
  unsigned long start = millis();

  while (wheels.isBusy() && (millis() - start < timeoutMs)) {
    wheels.updateMotion();
    delay(2);
  }

  // Fallback stop when timeout occurs or motion already finished.
  wheels.stop();
}

void moveBackAndAvoidObstacle() {
  // Short retreat to gain room before turning.
  wheels.goBackCm(8.0f);
  runMotionUntilDone(2500);

  bool turnRight = chooseRightEscapeSide();
  if (!SONAR_HIGHER_ANGLE_IS_RIGHT) {
    turnRight = !turnRight;
  }

  if (turnRight) {
    wheels.turnRightDeg(45.0f);
  } else {
    wheels.turnLeftDeg(45.0f);
  }

  runMotionUntilDone(3000);
}

void goLoop() {
  if (moving) {
    wheels.forward(); // Zwykła jazda do przodu bez limitu dystansu
  }
}

void setup() {
  pinMode(TRIG, OUTPUT);
  pinMode(ECHO, INPUT);

  Serial.begin(9600);
  Serial.setTimeout(200);
  lcd.init();
  lcd.backlight();
  lcd.setCursor(0, 0);
  
  // Jeśli na 2,3 oraz 4,5 auto jedzie do tyłu zamiast do przodu,
  // zamieniamy kable kierunku miejscami: (3, 2) oraz (5, 4)
  wheels.attach(4,5,6,7,10,11);
  wheels.setSpeed(120);

  // Setup encoders on A0 (left) and A1 (right) using PinChangeInterrupt
  pinMode(LEFT_ENCODER_PIN, INPUT_PULLUP);
  pinMode(RIGHT_ENCODER_PIN, INPUT_PULLUP);
  attachPCINT(digitalPinToPCINT(LEFT_ENCODER_PIN), leftEncoderISR, RISING);
  attachPCINT(digitalPinToPCINT(RIGHT_ENCODER_PIN), rightEncoderISR, RISING);
  wheels.resetEncoders();

  serwo.attach(SERVO);
  serwo.write(SONAR_CENTER_ANGLE);

  // Inicjalizacja odbyła się wyżej
}

void loop() {
  // Ciągłe wywoływanie goForward psuło by sygnał PWM dla kół
  // wheels.goForward(100); 
  
  moveTicker.check();
  sonarTicker.check();
}