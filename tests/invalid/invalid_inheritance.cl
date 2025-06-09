class A inherits B {
   foo(): Int { 0 };
};

class B inherits C {
   foo(): Int { 0 };
};

class C inherits A {
   foo(): Int { 0 };
};

class Main {
   main(): Object { new A.foo() };
};
