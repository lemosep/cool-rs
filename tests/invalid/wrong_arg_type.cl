class Main {
   main(): Object {
       let x : Int <- 5 in {
           out_string(x)  -- trying to pass an Int to out_string (expects String)
       }
   };
};
