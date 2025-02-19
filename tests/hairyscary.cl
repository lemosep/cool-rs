class Foo inherits Bazz {
     a : Rizz <- case self of
		      n : Rizz => (new Bar);
		      n : Foo => (new Rizz);
		      n : Bar => n;
   	         esac;

     b : Int <- a.doh() + g.doh() + doh() + printh();

     doh() : Int { (let i : Int <- h in { h <- h + 2; i; } ) };

};

class Bar inherits Rizz {

     c : Int <- doh();

     d : Object <- printh();
};


class Rizz inherits Foo {

     e : Bar <- case self of
		  n : Rizz => (new Bar);
		  n : Bar => n;
		esac;

     f : Int <- a@Bazz.doh() + g.doh() + e.doh() + doh() + printh();

};

class Bazz inherits IO {

     h : Int <- 1;

     g : Foo  <- case self of
		     	n : Bazz => (new Foo);
		     	n : Rizz => (new Bar);
			n : Foo  => (new Rizz);
			n : Bar => n;
		  esac;

     i : Object <- printh();

     printh() : Int { { out_int(h); 0; } };

     doh() : Int { (let i: Int <- h in { h <- h + 1; i; } ) };
};

(* scary . . . *)
class Main {
  a : Bazz <- new Bazz;
  b : Foo <- new Foo;
  c : Rizz <- new Rizz;
  d : Bar <- new Bar;

  main(): String { "do nothing" };

};