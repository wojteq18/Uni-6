#define LED 13

volatile char serialInput;
volatile int delayTime;


void setup() {
  pinMode(LED, OUTPUT);
  Serial.begin(9600);
  delayTime = 300;
}

void loop() {
  while(Serial.available())
  {
    serialInput = Serial.read();
    switch(serialInput)
    {
      case 'a' : 
                  kropka(1);
                  kreska(1);
                  koniec_litery();
                  break;
      case 'b' : 
                  kreska(1);
                  kropka(3);
                  koniec_litery();
                  break;  
      case 'c' : 
                  kreska(1);
                  kropka(1);
                  kreska(1);
                  kropka(1);
                  koniec_litery();
                  break;   
      case 'd' : 
                  kreska(1);
                  kropka(2);
                  koniec_litery();
                  break;
      case 'e' :
                  kropka(1);
                  koniec_litery();
                  break;
      case 'f' : 
                  kropka(2);
                  kreska(1);
                  kropka(1);
                  koniec_litery();
                  break;   
      case 'g' : 
                  kreska(2);
                  kropka(1);
                  koniec_litery();
                  break;  
      case 'h' : 
                  kropka(4);
                  koniec_litery();
                  break; 
      case 'i' : 
                  kropka(2);
                  koniec_litery();
                  break;
      case 'j' : 
                  kropka(1);
                  kreska(3);
                  koniec_litery();
                  break; 
      case 'k' : 
                  kreska(1);
                  kropka(1);
                  kreska(1);
                  koniec_litery();
                  break;   
      case 'l' : 
                  kropka(1);
                  kreska(1);
                  kropka(2);
                  koniec_litery();
                  break;  
      case 'm' : 
                  kreska(2);
                  koniec_litery();
                  break; 
      case 'n' : 
                  kreska(1);
                  kropka(1);
                  koniec_litery();
                  break;   
      case 'o' : 
                  kreska(3);
                  koniec_litery();
                  break;    
      case 'p' : 
                  kropka(1);
                  kreska(2);
                  kropka(1);
                  koniec_litery();
                  break;   
      case 'q' : 
                  kreska(2);
                  kropka(1);
                  kreska(1);
                  koniec_litery();
                  break;  
      case 'r' : 
                  kropka(1);
                  kreska(1);
                  kropka(1);
                  koniec_litery();
                  break;    
      case 's' : 
                  kropka(3);
                  koniec_litery();
                  break; 
      case 't' : 
                  kreska(1);
                  koniec_litery();
                  break; 
      case 'u' : 
                  kropka(2);
                  kreska(1);
                  koniec_litery();
                  break;  
      case 'v' : 
                  kropka(3);
                  kreska(1);
                  koniec_litery();
                  break;  
      case 'w' : 
                  kropka(1);
                  kreska(2);
                  koniec_litery();
                  break;    
      case 'x' : 
                  kreska(1);
                  kropka(2);
                  kreska(1);
                  koniec_litery();
                  break;    
      case 'y' : 
                  kreska(1);
                  kropka(1);
                  kreska(2);
                  koniec_litery();
                  break; 
      case 'z' : 
                  kreska(2);
                  kropka(2);
                  koniec_litery();
                  break;        
      case ' ' :
                  spacja();                                                                                                                                                                                                  
      default  : Serial.println("?");
    }
  }
}


void doTheBlinking(int times) {
  for(int i=0; i<times; i++) {
    digitalWrite(LED, HIGH);
    delay(delayTime);
    digitalWrite(LED, LOW);    
    delay(delayTime);
  }  
}

void kropka(int times) {
  delayTime = 100;
  doTheBlinking(times);
  delay(100);
}

void kreska(int times) {
  delayTime = 300;
  doTheBlinking(times);
  delay(100);
}

void koniec_litery() {
  delay(800);
} 

void spacja() {
  delay(600);
}
