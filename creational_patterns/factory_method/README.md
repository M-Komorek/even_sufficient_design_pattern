# Factory Method
Factory Method pattern is a **creational pattern** that uses **factory methods** to deal with the problem of creating objects without having to specify the exact class of the object that will be created. This is done by creating objects by calling a factory method instead rather than by calling a constructor.

Many designs start by using Factory Method (less complicated and more customizable via subclasses) and evolve toward **Abstract Factory**, **Prototype**, or **Builder** (more flexible, but more complicated).

## Advantages
- Avoiding close association of the creator with specific products.
- **Single Responsibility Principle** - the ability to move the product creation code to one place in the program, which will make the code easier to maintain.
- **Open/Closed Principle** - the ability to extend the program with new types of products without breaking the existing client code.

## Disadvantages
- A large amount of code resulting from many subclasses.
