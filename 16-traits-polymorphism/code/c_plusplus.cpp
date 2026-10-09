#include <iostream>
using namespace std;

class Eat {
public:
   void eat_dinner() {
   }
};

class Person : public Eat {
public:
   string name;

   Person(string name) {
       this->name = name;
   }

   void eat_dinner() {
       cout << name << " Yummy\n";
   }
};

class Cat : public Eat {
public:
   string name;

   Cat(string name) {
       this->name = name;
   }

   void eat_dinner() {
       cout << name << " Num Num Num\n";
   }
};

int main() {
   Person perr("Coco");
   perr.eat_dinner();

   Cat catt("Mr.Joe");
   catt.eat_dinner();

   return 0;
}
