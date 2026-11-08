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

    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao),) {
        self.permissoes = permissoes;
    }

    // APR De Dados
    fn stat(&self) {
        println!("\nARQUIVO:"); //I wanna put the archive name here later wih a {}
        println!("Nome: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!(
            "Permissoes: ({}{}{} / {}{}{})", self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal(), self.permissoes.0.perm_escrita(), self.permissoes.1.perm_escrita(), self.permissoes.2.perm_escrita(),);
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
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
    fn octal(&self) -> u8 {
            let mut valor: u8 = 0;
            if self.leitura {
                valor = valor + 4;
            }
            if self.escrita {
                valor = valor + 2;
            }
            if self.execucao {
                valor = valor + 1;
            }

            valor
        }

    fn perm_escrita(&self) -> String {
        let mut p = String::new();

        if self.leitura {
            p.push('r');
        } else {
            p.push('-');
        }
        if self.escrita {
            p.push('w');
        } else {
            p.push('-');
        }
        if self.execucao {
            p.push('x');
        } else {
            p.push('-');
        }
        p
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

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome_arquivo: String){
        for i in 0..self.arquivos.len() {
            if self.arquivos[i].nome == nome_arquivo {
                self.arquivos.remove(i);
                println!("Arquivo {} removido com sucesso.", nome_arquivo);
                return;
            }
        }
        println!("Arquivo {} nao encontrado.", nome_arquivo);
    }

    fn listar_conteudo(&self){
        for arquivo in &self.arquivos {
            println!("{}", arquivo.nome); // adicionar arquivo.tamanho?
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

    fn adiciona_grupo(&mut self, gid: u16) {
        self.grupos_ids.push(gid);
    }

    fn remove_grupo(&mut self, gid: u16) {
        for i in 0..self.grupos_ids.len() {
            if self.grupos_ids[i] == gid {
                self.grupos_ids.remove(i);
                println!("Grupo {} removido com sucesso.", gid);
                return;
            }
        }
        println!("Grupo {} nao encontrado.", gid);
    }

    fn listar_grupos(&self) {
        for i in &self.grupos_ids {
            println!("{}", i);
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

    fn adiciona_membro(&mut self, uid:u16) {
        self.membros_uids.push(uid);
    }
    fn remove_membro(&mut self, uid:u16){
        for i in 0..self.membros_uids.len(){
            if self.membros_uids[i] == uid{
                self.membros_uids.remove(i);
                return;
            }
        }
    }
    fn listar_membros(&self){
        for i in &self.membros_uids {
            println!("{}", i);
        }
    }
}

fn main() {
    // estado que vive durante todo o programa
    let mut diretorio = Diretorio::new(
        String::from("raiz"),
        (
            Permissao::new(true, true, true),
            Permissao::new(true, true, false),
            Permissao::new(true, false, false),
        ),
        1000,
    );
    let mut usuario = Usuario::new(String::from("ana"), 1000);
    let mut grupo = Grupo::new(String::from("users"), 1000);
    let mut arquivo = Arquivo::new(String::from("vazio"), 0, 1000, 1000);

    loop {
        println!("\n=== MENU ===");
        println!("1 - criar arquivo");
        println!("2 - editar permissao do arquivo");
        println!("3 - apagar arquivo do diretorio");
        println!("4 - listar conteudo do diretorio");
        println!("5 - adicionar arquivo ao diretorio");
        println!("6 - adicionar grupo ao usuario");
        println!("7 - remover grupo do usuario");
        println!("8 - listar grupos do usuario");
        println!("9 - adicionar membro ao grupo");
        println!("10 - remover membro do grupo");
        println!("11 - listar membros do grupo");
        println!("12 - stat do arquivo");
        println!("0 - sair");
        print!("Escolha uma opcao: ");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();

        // Tratando o input da opção
        let opcao: u32 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um numero valido.");
                continue;
            }
        };

        // AQUI COMEÇA O IF / ELSE
        if opcao == 1 {
            println!("nome do arquivo:");
            let mut nome = String::new();
            io::stdin().read_line(&mut nome).unwrap();
            let nome = nome.trim().to_string();

            println!("tamanho em bytes:");
            let mut tam = String::new();
            io::stdin().read_line(&mut tam).unwrap();
            let tam: u64 = tam.trim().parse().unwrap();

            arquivo = Arquivo::new(nome, tam, 1000, 1000);
            println!("Arquivo criado!");

        } else if opcao == 2 {
            let p_dono = Permissao::new(true, false, false);
            let p_grupo = Permissao::new(true, false, false);
            let p_outros = Permissao::new(true, false, false);

            arquivo.alterar_permissao((p_dono, p_grupo, p_outros));
            println!("Permissoes alteradas!");

        } else if opcao == 3 {
            println!("nome do arquivo a remover:");
            let mut nome = String::new();
            io::stdin().read_line(&mut nome).unwrap();
            diretorio.remove_arquivo(nome.trim().to_string());

        } else if opcao == 4 {
            diretorio.listar_conteudo();

        } else if opcao == 5 {
            let copia = Arquivo::new(
                arquivo.nome.clone(),
                arquivo.tamanho,
                arquivo.uid_dono,
                arquivo.gid_grupo,
            );
            diretorio.adiciona_arquivo(copia);
            println!("Arquivo adicionado ao diretorio!");

        } else if opcao == 6 {
            println!("gid a adicionar:");
            let mut g = String::new();
            io::stdin().read_line(&mut g).unwrap();
            usuario.adiciona_grupo(g.trim().parse().unwrap());
            println!("Grupo adicionado ao usuario!");

        } else if opcao == 7 {
            println!("gid a remover:");
            let mut g = String::new();
            io::stdin().read_line(&mut g).unwrap();
            usuario.remove_grupo(g.trim().parse().unwrap());

        } else if opcao == 8 {
            usuario.listar_grupos();

        } else if opcao == 9 {
            println!("uid a adicionar:");
            let mut u = String::new();
            io::stdin().read_line(&mut u).unwrap();
            grupo.adiciona_membro(u.trim().parse().unwrap());
            println!("Membro adicionado ao grupo!");

        } else if opcao == 10 {
            println!("uid a remover:");
            let mut u = String::new();
            io::stdin().read_line(&mut u).unwrap();
            grupo.remove_membro(u.trim().parse().unwrap());

        } else if opcao == 11 {
            grupo.listar_membros();

        } else if opcao == 12 {
            arquivo.stat();

        } else if opcao == 0 {
            println!("Saindo...");
            break;

        } else {
            println!("Opcao invalida!");
        }
    }
}
