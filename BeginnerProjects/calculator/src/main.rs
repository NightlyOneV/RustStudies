use std::io;

fn sum(x: f32, y: f32) -> (f32, &'static str) {
    return (x + y, "Sum:");
}

fn sub(x: f32, y: f32) -> (f32, &'static str) {
    return (x - y, "Subtraction:");
}

fn div(x: f32, y: f32) -> (f32, &'static str) {
    return (x / y, "Division:");
}

fn modl(x: f32, y: f32) -> (f32, &'static str) {
    return (x % y, "Mod:");
}

fn main(){
    let mut _num1 = String::new();
    let mut _num2 = String::new();
    
    match io::stdin().read_line(&mut _num1) {
        Ok(_) => {
            println!("Number 1: {}", _num1);
        }
        Err(error) => println!("Erro: {}", error)
    }
    
    match io::stdin().read_line(&mut _num2) {
        Ok(_) => {
            println!("Number 2: {}", _num2);
        }
        Err(error) => println!("Erro: {}", error)
    }
    
    let num1: f32 = _num1.trim().parse().unwrap();
    let num2: f32 = _num2.trim().parse().unwrap();
    
    let mut result: Vec<(f32,&str)> = vec![];
    
    result.push(sum(num1, num2));
    result.push(sub(num1, num2));
    result.push(div(num1, num2));
    result.push(modl(num1, num2));
    
    for x in result {
        println!("{} {}", x.1, x.0)
    }
    
}