use std::io;

#[derive(Debug)]
struct Processo {
    pid: i32,
    men_size: i32,
    time_execution: i32,
}

fn ler_entrada() -> Option<(i32, i32)> {
    println!("Digite o tamanho de memória e o tempo de execução do processo, ou 'sair' para finalizar:");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a linha");

    if entrada.trim() == "sair" {
        return None;
    }

    let partes: Vec<&str> = entrada.trim().split_whitespace().collect();
    if partes.len() != 2 {
        println!("Entrada inválida. Por favor, forneça dois números.");
        return None;
    }

    let men_size: i32 = match partes[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Falha ao parsear o tamanho de memória. Certifique-se de inserir um número válido.");
            return None;
        }
    };

    let time_execution: i32 = match partes[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Falha ao parsear o tempo de execução. Certifique-se de inserir um número válido.");
            return None;
        }
    };

    if time_execution < 30 || time_execution > 90 {
        println!("O tempo de execução deve estar entre 30 e 90 segundos.");
        return None;
    }

    Some((men_size, time_execution))
}

fn main() {
    let mut pilha: Vec<Processo> = Vec::new();
    let mut pid_counter = 0;

    loop {
        if let Some((men_size, time_execution)) = ler_entrada() {
            let processo = Processo {
                pid: pid_counter,
                men_size,
                time_execution,
            };
            pilha.push(processo);
            pid_counter += 1;
        } else {
            break;
        }
    }

    println!("PIDs dos processos na pilha:");
    for processo in &pilha {
        println!("{}", processo.pid);
    }
}
