use std::io;

// STRUCTS COMECAM AQUI
// STR arquivo
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16,
}
// STR permissoes
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}
// STR Diretorio
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}
// STR usuario
struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>,
}
// STR Grupo
struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16>,
}


// IMPLS comecam aquiii
impl Arquivo {
    // NEW
    fn new(nome: String, tamanho: u64, uid_dono: u16, gid_grupo: u16,) -> Self {
        Self {
            nome,
            tamanho,
            permissoes: (Permissao {leitura: true, escrita: true, execucao: false,}, Permissao {leitura: true, escrita: true, execucao: false,}, Permissao {leitura: true, escrita: false, execucao: false, },),
            uid_dono,
            gid_grupo,
        }
    }

    // APR DE DADOS
    fn apresentar(&self) {
        println!("\n===== DADOS DO ARQUIVO =====");
        println!("Nome: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("Permissões do dono:");
        println!("Leitura: {}", self.permissoes.0.leitura);
        println!("Escrita: {}", self.permissoes.0.escrita);
        println!("Execução: {}", self.permissoes.0.execucao);
        println!("Permissões do grupo:");
        println!("Leitura: {}", self.permissoes.1.leitura);
        println!("Escrita: {}", self.permissoes.1.escrita);
        println!("Execução: {}", self.permissoes.1.execucao);
        println!("Permissões de outros:");
        println!("Leitura: {}", self.permissoes.2.leitura);
        println!("Escrita: {}", self.permissoes.2.escrita);
        println!("Execução: {}", self.permissoes.2.execucao);
        println!("uid dono: {}", self.uid_dono);
        println!("gid grupo: {}", self.gid_grupo);
    }
}
impl Permissao{
    fn new(leitura: bool, escrita:bool, execucao:bool) -> Self {
        Self {
            leitura,
            escrita,
            execucao,

        }
    }

}
impl Diretorio {
    fn new(nome: String, permissoes:(Permissao, Permissao, Permissao), uid_dono: u16) -> Self {
        Self {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono,
        }
    }

}
impl Usuario {
    fn new(nome: String, uid: u16) -> Self {
        Self {
            nome,
            uid,
            grupos_ids: Vec::new(),
        }
    }

}
impl Grupo {
    fn new(nome: String, gid: u16) -> Self {
        Self{
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }
}
fn main() {
    let mut nome = String::new();

    println!("Digite seu nome:");
    io::stdin().read_line(&mut nome).expect("Erro ao ler nome");

    let nome = nome.trim().to_string();

    //input do tamanho

    let mut tamanho = String::new();

    println!("Digite o tamanho do arquivo que vc quer fofa:");
    io::stdin().read_line(&mut tamanho).expect("Erro");

    let tamanho: u64 = tamanho.trim().parse().expect("Valor inválido");

    let uid_dono: u16 = 1000;

    let gid_grupo: u16 = 1000;

    // CRIANDO A STRUCT COM NEW
    let arquivo1 = Arquivo::new(nome, tamanho, uid_dono, gid_grupo);

    arquivo1.apresentar();
}
