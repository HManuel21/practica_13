/* fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    }else{
        n * factorial(n - 1) 
    }
}
fn main() {
    let n = 5 ;
    let resultado = factorial(n);
    println!("El factorial de {} es: {}", n, resultado);
}
 */

/*  fn es_palindroma(palabra: &str) -> bool {
    let palabra = palabra.to_lowercase(); 
    let palabra = palabra.chars().collect::<Vec<char>>();

    fn verificar_palindroma(palabra: &Vec<char>, inicio: usize, fin: usize) -> bool {
        if inicio >= fin { 
            return true;
        }

        if palabra[inicio] != palabra[fin] {
            return false;
        }

        verificar_palindroma(&palabra, inicio + 1, fin -1)
    }

    verificar_palindroma(&palabra, 0, palabra.len() -1)
 }

 fn main() {
    println!("Ingrese la palabra");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let  palabra = input.trim();

    if es_palindroma(palabra) {
        println!("La palabra es palindroma.");
    }else {
        println!("La palabra no es palindroma.");
    }
 } */

 fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }

 }

 fn main() {
    println!("Ingrese la cantidad de numeros de la sucesion de Fibonaccio que decea ver: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada. ");
    let n: u32 = input.trim().parse().expect("Error al analizar la entrada.");

    if n == 0 {
        println!("No se mostrara numeros. ");
    }else {
        println!("Sucesion de Fibonacci de los primeros {} numeros ", n);
        for i in 0..n {
            println!("{}", fibonacci(i));
        }
        println!();
    }
 }