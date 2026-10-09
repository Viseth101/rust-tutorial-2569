interface Eat {
   void eat_dinner();
}

class Person implements Eat{
   String name ;

   Person(String name ){
       this.name = name ; 
   }

   public  void eat_dinner(){
       System.out.println(name +"  Yummy");
   }
}

class Cat implements Eat{
   String name ;

   Cat(String name ){
       this.name = name ; 
   }

   public  void eat_dinner(){
       System.out.println(name + " Num Num NUm");
   }
}

public class javacode {
   public static void main(String[] args) {
       Person perr = new Person("Coco");
       perr.eat_dinner();
       Cat catt = new Cat("Mr.Joe");
       catt.eat_dinner();
   }
   
}