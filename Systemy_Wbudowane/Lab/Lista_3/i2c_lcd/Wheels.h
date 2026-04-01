#ifndef Wheels_h
#define Wheels_h
#include <Arduino.h>

class Wheels {
    public: 
        Wheels();
        void attachRight(int pinForward, int pinBack, int pinSpeed);
        void attachLeft(int pinForward, int pinBack, int pinSpeed);
        void attach(int pinRightForward, int pinRightBack, int pinRightSpeed,
                    int pinLeftForward, int pinLeftBack, int pinLeftSpeed);
        
        void forward();
        void forwardLeft();
        void forwardRight();
        void back();
        void backLeft();
        void backRight();
        void stop();
        void stopLeft();
        void stopRight();
        
        // Zmienione funkcje na nieblokujące
        void goForward(int cm);
        void goBack(int cm);
        void update(); // NOWA FUNKCJA: do wywoływania w loop()
        int getRemainingDistance(); // NOWA FUNKCJA: dla LCD

        void setSpeed(uint8_t);
        void setSpeedRight(uint8_t);
        void setSpeedLeft(uint8_t);

        // Gettery dla LCD
        int getSpeedLeft();
        int getSpeedRight();
        int getDirection(); // 1: przód, -1: tył, 0: stop

    private: 
        int pinsRight[3];
        int pinsLeft[3];
        
        // Zmienne do asynchronicznego ruchu
        unsigned long movementStartTime;
        unsigned long movementDuration;
        bool isMovingDistance;
        int currentDirection;
        int speedL;
        int speedR;
};
#endif