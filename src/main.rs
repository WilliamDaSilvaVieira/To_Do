use std::{
    error::Error,
    fs::{write, File},
    io::{self, Read},
};

fn main() {
    println!("Bem vindo ao To_Do!");

    let mut to_do: Vec<String> = Vec::new();

    let mut text = file().unwrap_or_default();

    println!("\n\nLista:\n{}\n\n", text);

    let option: char = loop {
        println!(
            "Deseja adicionar uma tarefa, apagar ou sair? (Adicionar, + | Apagar, - | Sair, =)"
        );
        println!("ATENÇÃO: A OPÇÃO 'Apagar' IRA APAGAR TODA A LISTA!");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Falha ao ler a linha!");

        match x.trim().parse() {
            Ok(x) => match x {
                '+' | '-' | '=' => break x,
                _ => {
                    eprintln!("Opção invalida!!");
                    continue;
                }
            },
            Err(_) => {
                eprintln!("Opção invalida!!");
                continue;
            }
        }
    };

    if option == '+' {
        loop {
            let mut tarefa = String::new();
            println!("Digite uma tarefa para adicionar. (. para terminar)");
            io::stdin()
                .read_line(&mut tarefa)
                .expect("Falha ao ler a linha!");

            if tarefa.trim() == "" {
                continue;
            }
            if tarefa.to_lowercase().trim() == "." {
                break;
            } else {
                to_do.push(tarefa.trim().to_string());
            }
        }

        for i in &to_do {
            text.push_str(i);
            text.push('\n');
        }
    }
    if option == '-' {
        text.clear();
    }

    write("to_do.txt", &text).expect("Falha ao gravar!");
    println!("\n\nLista:\n{text}");
}

fn file() -> Result<String, Box<dyn Error>> {
    let mut f = File::open("to_do.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
