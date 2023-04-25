fn main() {
    
    indice();
    matriz();
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    let cor_cmyk: Color = Color::CmykColor{cyan: 100, yellow:50, magenta:150, black:200};
    println!("cor: {}", cores(cor_cmyk));

    println!("{}", conteudo_opcional());

    vetores();
    conta_corrente();
}

fn indice() {
    let notas: [f32; 3] = [10f32, 8.0, 9.5];

    for indice in 0..notas.len() {
        println!("A nota {} é {}", indice + 1, notas[indice]);
    }

    let notas2: [f32; 3] = [6.5; 3];

    for indice in 0..notas2.len(){
        
        println!("A nota {} é {}", indice + 1, notas2[indice]);
    }
}

fn matriz(){
    let matriz: [[f32;3];2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for indice in linha {
            println!("{}", indice)
        }
    }
}

#[allow(dead_code)]
enum DiaDaSemana{
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana)-> bool{
    return match dia_da_semana{
        DiaDaSemana:: Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor{cyan: u8, yellow: u8, magenta: u8, black: u8}
}

fn cores(cor: Color)->&'static str{

    return match cor{
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0) | Color::CmykColor { cyan:_, yellow:_, magenta:_, black:255 } => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CmykColor { cyan:_, yellow:_, magenta:_, black:_ } => "CMYK desconhecido"
    };
}

fn conteudo_opcional() -> String{
    let conteudo_arquivo = ler_arquivo(String::from(""));
    
    println!("{:?}", &conteudo_arquivo);
    
    if let Some(valor) = &conteudo_arquivo {
        println!("Agora há um valor")
    };

    return match conteudo_arquivo{
        Some(valor) => valor,
        None => String::from("Arquivo nao existe")
    };
}

fn ler_arquivo(caminho_arquivo: String)-> Option<String> {
    Some(String::from("Conteudo do Arquivo"))
}

fn vetores(){
    let mut notas: Vec<f32> = Vec::new();
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("{:?}", notas);

    println!("Nota 7 = {}", match notas.get(6){
        Some(n) => *n,
        None => 0.0
    });

    for nota in &notas {
        println!("Nota = {}", nota);
    }
}

struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64){
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

fn conta_corrente(){
    let titular: Titular = Titular { 
        nome: String::from("Nathan"),
        sobrenome: String::from("Gurgel") 
    };

    let mut conta: Conta = Conta{
        titular: titular,
        saldo:100.0
    };

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo)
}