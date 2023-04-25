const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1;

fn main(){
    escopo();
    sombra();
    println!("Soma = {}", soma(2, 2));
    condicionais();
    repeticoes();
    pattern_matching();
}

fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn escopo(){
    println!("PI = {}", PI);
    println!("variavel_global = {}", VARIAVEL_GLOBAL);

    let variavel:i32 = 127;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    let variavel:i32 = 128;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let boolean = false;
    println!("Tamanho boolean = {}", std::mem::size_of_val(&boolean));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn sombra(){
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("a = {}", a);
}

fn condicionais(){
    let idade: u8 = 18;
    let eh_maior = idade >= 18;

    let condicao = if eh_maior {"maior"} else {"menor"};

    println!("É {} de idade", condicao);

    let linguagem = "";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O propósito de {} é {}", linguagem, proposito);
}

fn repeticoes(){
    let multiplicador:u8 = 5;
    
    let mut contador:u8 = 0;
    while contador < 10{
        contador +=1;

        if contador == 5{
            continue;
        }

        
    }

    contador = 0;

    loop {
        contador += 1;

        if contador == 10{
            break;
        }
    }

    for i in 1..11{
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn ownership(){
    let mut uma_string = String::from("Nathan");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String){
    string.push_str("Gurgel");
    println!("{}", string);
}

fn pattern_matching(){
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        })
    }
}

fn erros(){
    match resultado() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Código de erro = {}", numero)
    }
}

fn resultado() -> Result<String, u8>{
    Err(42)
}