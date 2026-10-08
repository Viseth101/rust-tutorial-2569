class Person:
    def __init__(self, name):
        self.name = name

    def eat_dinner(self):
        print(self.name +" Yummy")


class Cat:
    def __init__(self, name):
        self.name = name

    def eat_dinner(self):
        print(self.name +" Num Num Num")


perr = Person("Coco")
perr.eat_dinner()

catt = Cat("Mr.Joe")
catt.eat_dinner()