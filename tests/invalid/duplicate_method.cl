class A {
   foo(x: Int): Int { x };
   foo(x: Int): Int { x + 1 };  -- duplicate method name+arity in the same class
};

class Main {
   main(): Object { new A.foo(5) };
};
