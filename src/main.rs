use std::io;

static ESCOLA: &str = "PUCPR";

// STRUCT
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (bool, bool, bool),
    uid_dono: u16,
    gid_grupo: u16,
}

// IMPLEMENTAÇÃO

impl Arquivo {
    // NEW
    fn new(
        nome: String,
        tamanho: u64,
        permissoes: (bool, bool, bool),
        uid_dono: u16,
        gid_grupo: u16,
    ) -> Self {
        Self {
            nome,
            tamanho,
            permissoes,
            uid_dono,
            gid_grupo,
        }
    }

    // MÉTODO PARA APRESENTAR OS DADOS
    fn apresentar(&self) {
        println!("\n===== DADOS DO ARQUIVO =====");
        println!("Nome: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("Leitura: {}", self.permissoes.0);
        println!("Escrita: {}", self.permissoes.1);
        println!("Execução: {}", self.permissoes.2);
        println!("gid dono: {}", self.gid_grupo);
        println!("uid grupo: {}", self.uid_dono);
        println!(": {}", ESCOLA);
    }
}

fn main() {
    // INPUT DO NOME
    let mut nome = String::new();

    println!("Digite seu nome:");
    io::stdin().read_line(&mut nome).expect("Erro ao ler nome");

    let nome = nome.trim().to_string();

    //input do tamanho

    let mut tamanho = String::new();

    println!("Digite o tamanho do arquivo que vc quer fofa:");
    io::stdin().read_line(&mut tamanho).expect("Erro");

    let tamanho: u64 = tamanho.trim().parse().expect("Valor inválido");

    let permissoes = (true, true, false);

    let uid_dono: u16 = 1000;

    let gid_grupo: u16 = 1000;

    // CRIANDO A STRUCT COM NEW
    let arquivo1 = Arquivo::new(nome, tamanho, permissoes, uid_dono, gid_grupo);

    // CHAMANDO O MÉTODO
    arquivo1.apresentar();
}
