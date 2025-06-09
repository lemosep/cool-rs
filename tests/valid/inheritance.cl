class A {
   foo(): Int { 10 };
};

class B inherits A {
   foo(): Int { 20 };
   bar(): Int { 
       let a : A <- new A in
         a.foo()  -- should return 10
   };
};

class C inherits B {
   bar(): Int {
       let b : B <- new B in
         b.foo()  -- inherited override returns 20
   };
};

class Main inherits IO {
   main(): Object {
       out_int(new C.bar())
   };
};
