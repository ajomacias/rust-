fn main() {
    /*
        ----------
        let lo = 12;  -Bad form :(
        lo = 13;

        let mut lo = 12;  -Correct form :) 
        lo = 13;
        -----------
        let mut spaces = "   "; -Bad form for the type of var change :(
        spaces = spaces.len();

        let spaces = "    ";
        let spaces = spaces.len(); -Correct form, we are use shadow :)
        ------------

        -Example is using shadow and ambito interno.

        let x = 12;
        let x = x + 3;
        println!("the value to var x is => {}" , x);
        {                                              -This is a ambito interno, lo que pase dentro no afecta a lo de afuera en este                            
          let x = 10;                                   caso usando sombreado.
          println!("the value of var x is => {}", x );
        }
        println!("the value to var x is => {}" , x);
        ------------
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
    */
    
    let sum : i8 = 12;
    let sum : i8  = sum - 100; 
    let _boleano : bool = true;

    let float : f64 = 12.12;
    {
        let float : f64 = float - 12.0;
        println!("the value pf variable float is => {}", float );
    }
    

    println!("the value pf variable float is => {}", float );
    
    println!("the value for var sum is => {}",sum);
    
   //let character : char = "T"; //error this is simple comillas :(
    let character : char = '✌';
    println!()character;
    
}