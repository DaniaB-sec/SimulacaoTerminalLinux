#![allow(dead_code)]



// TRABALHO FEITO POR JULIA TREMBULACK AGOSTINHO E DANIELLE BECK.

// TURMA CIBERSEGURANCA

//TURNO: NOITE



use std::io;

use std::thread::sleep;

use std::time::Duration;



// STRUCTS COMECAM AQUI

struct Arquivo {

    nome: String,

    tamanho: u64,

    permissoes: (Permissao, Permissao, Permissao),

    uid_dono: u16,

    gid_grupo: u16,

}

struct Permissao {

    leitura: bool,

    escrita: bool,

    execucao: bool,

}

struct Diretorio {

    nome: String,

    arquivos: Vec<Arquivo>,

    permissoes: (Permissao, Permissao, Permissao),

    uid_dono: u16,

}

struct Usuario {

    nome: String,

    uid: u16,

    grupos_ids: Vec<u16>,

}

struct Grupo {

    nome: String,

    gid: u16,

    membros_uids: Vec<u16>,

}



// IMPLS comecam aquiii

impl Arquivo {

    fn new(nome: String, tamanho: u64, uid_dono: u16, gid_grupo: u16) -> Self {

        Self {

            nome,

            tamanho,

            permissoes: (

                Permissao { leitura: true, escrita: true, execucao: false },

                Permissao { leitura: true, escrita: true, execucao: false },

                Permissao { leitura: true, escrita: false, execucao: false },

            ),

            uid_dono,

            gid_grupo,

        }

    }



    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {

        self.permissoes = permissoes;

    }



    fn stat(&self) {

        println!("\nARQUIVO:");

        println!("Nome: {}", self.nome);

        println!("Tamanho: {} bytes", self.tamanho);

        println!(

            "Permissoes: ({}{}{} / {}{}{})",

            self.permissoes.0.octal(),

            self.permissoes.1.octal(),

            self.permissoes.2.octal(),

            self.permissoes.0.perm_escrita(),

            self.permissoes.1.perm_escrita(),

            self.permissoes.2.perm_escrita(),

        );

        println!("Uid: {}", self.uid_dono);

        println!("Gid: {}", self.gid_grupo);

    }

}



impl Permissao {

    fn new(leitura: bool, escrita: bool, execucao: bool) -> Self {

        Self { leitura, escrita, execucao }

    }



    fn octal(&self) -> u8 {

        let mut valor: u8 = 0;

        if self.leitura { valor = valor + 4; }

        if self.escrita { valor = valor + 2; }

        if self.execucao { valor = valor + 1; }

        valor

    }



    fn perm_escrita(&self) -> String {

        let mut p = String::new();

        if self.leitura { p.push('r'); } else { p.push('-'); }

        if self.escrita { p.push('w'); } else { p.push('-'); }

        if self.execucao { p.push('x'); } else { p.push('-'); }

        p

    }

}



impl Diretorio {

    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Self {

        Self { nome, arquivos: Vec::new(), permissoes, uid_dono }

    }



    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {

        self.arquivos.push(arquivo);

    }



    fn remove_arquivo(&mut self, nome_arquivo: String) {

        for i in 0..self.arquivos.len() {

            if self.arquivos[i].nome == nome_arquivo {

                self.arquivos.remove(i);

                println!("Arquivo {} removido com sucesso.", nome_arquivo);

                return;

            }

        }

        println!("Arquivo {} nao encontrado.", nome_arquivo);

    }



    fn listar_conteudo(&self) {

        for arquivo in &self.arquivos {

            println!("{}, {}", arquivo.nome, arquivo.tamanho);

        }

    }

}



impl Usuario {

    fn new(nome: String, uid: u16) -> Self {

        Self { nome, uid, grupos_ids: Vec::new() }

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

        Self { nome, gid, membros_uids: Vec::new() }

    }



    fn adiciona_membro(&mut self, uid: u16) {

        self.membros_uids.push(uid);

    }



    fn remove_membro(&mut self, uid: u16) {

        for i in 0..self.membros_uids.len() {

            if self.membros_uids[i] == uid {

                self.membros_uids.remove(i);

                println!("Membro removido.");

                return;

            }

        }

        println!("Membro {} nao encontrado", uid);

    }



    fn listar_membros(&self) {

        for i in &self.membros_uids {

            println!("{}", i);

        }

    }

}



fn ler_input(texto: &str) -> String {

    println!("{}", texto);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Erro");

    input.trim().to_string()

}



fn menu() {

    let mut diretorios: Vec<Diretorio> = Vec::new();

    let mut arquivos: Vec<Arquivo> = Vec::new();

    let mut usuarios: Vec<Usuario> = Vec::new();

    let mut grupos: Vec<Grupo> = Vec::new();



    let mut proximo_uid_arquivo: u16 = 1000;



    loop {

        println!("\n============= MENU =============");

        println!("1 - criar arquivo");

        println!("2 - criar diretorio");

        println!("3 - criar usuario");

        println!("4 - criar grupo");

        println!("5 - listar arquivos");

        println!("6 - listar diretorios");

        println!("7 - listar usuarios");

        println!("8 - listar grupos");

        println!("9 - remover arquivo");

        println!("10 - remover diretorio");

        println!("11 - remover usuario");

        println!("12 - remover grupo");

        println!("13 - mudar permissoes ");

        println!("14 - stat do arquivo ");

        println!("15 - add usuario ao grupo ");

        println!("16 - listar membros de um grupo");

        println!("17 - remover usuario de um grupo ");

        println!("18 - ver grupos de usuario ");

        println!("0 - sair");



        let resposta = ler_input("Escolha uma opcao:");



        let opcao: u32;

        if let Ok(numero) = resposta.parse::<u32>() {

            opcao = numero;

        } else {

            println!("Digite um numero valido.");

            continue;

        }



        match opcao {

            1 => {

                let nome = ler_input("Nome do arquivo:");

                let tam_str = ler_input("Tamanho em bytes:");



                let tam: u64 = tam_str.parse().unwrap_or(0);

                if tam == 0 {

                    println!("Tamanho invalido. Use apenas numeros.");

                    continue;

                }



                arquivos.push(Arquivo::new(nome, tam, proximo_uid_arquivo, proximo_uid_arquivo));

                println!("Arquivo criado! (uid/gid = {})", proximo_uid_arquivo);

                proximo_uid_arquivo += 1;

            },



            2 => {

                let nome = ler_input("Nome do diretorio:");



                let p_dono = Permissao::new(true, true, false);

                let p_grupo = Permissao::new(true, true, false);

                let p_outros = Permissao::new(true, false, false);



                diretorios.push(Diretorio::new(nome, (p_dono, p_grupo, p_outros), 1000));

                println!("Diretorio criado!!");

            },



            3 => {

                let nome = ler_input("Nome do usuario:");

                let uid_str = ler_input("UID:");



                let uid: u16 = uid_str.parse().unwrap_or(0);

                if uid == 0 {

                    println!("UID invalido. Use apenas numeros.");

                    continue;

                }



                usuarios.push(Usuario::new(nome, uid));

                println!("Usuario criado! (uid = {})", uid);

            },



            4 => {

                let nome = ler_input("Nome do grupo:");

                let gid_str = ler_input("GID:");



                let gid: u16 = gid_str.parse().unwrap_or(0);

                if gid == 0 {

                    println!("GID invalido. Use apenas numeros.");

                    continue;

                }



                grupos.push(Grupo::new(nome, gid));

                println!("Grupo criado! (gid = {})", gid);

            },



            5 => {

                if arquivos.is_empty() {

                    println!("Nenhum arquivo criado.");

                } else {

                    for i in 0..arquivos.len() {

                        println!("{} ({} bytes, uid={}, gid={})",

                                 arquivos[i].nome,

                                 arquivos[i].tamanho,

                                 arquivos[i].uid_dono,

                                 arquivos[i].gid_grupo);

                    }

                }

            }



            6 => {

                if diretorios.is_empty() {

                    println!("Nenhum diretorio criado.");

                } else {

                    for i in 0..diretorios.len() {

                        println!("{} (uid={}, {} arquivo(s))",

                                 diretorios[i].nome,

                                 diretorios[i].uid_dono,

                                 diretorios[i].arquivos.len());

                    }

                }

            },



            7 => {

                if usuarios.is_empty() {

                    println!("Nenhum usuario criado.");

                } else {

                    for i in 0..usuarios.len() {

                        println!("{} (uid={})", usuarios[i].nome, usuarios[i].uid);

                    }

                }

            },



            8 => {

                if grupos.is_empty() {

                    println!("Nenhum grupo criado");

                } else {

                    for i in 0..grupos.len() {

                        println!("{} (gid={})", grupos[i].nome, grupos[i].gid);

                    }

                }

            },



            9 => {

                let nome = ler_input("Nome do arquivo pra remover:");

                let mut removido = false;  
                for i in 0..arquivos.len() {

                    if arquivos[i].nome == nome {

                        arquivos.remove(i);

                        println!("Arquivo removido");

                        removido = true;

                        break;

                    }

                }

                if !removido {

                    println!("Arquivo nao encontrado.");

                }

            },



            10 => {

                let nome = ler_input("Nome do diretorio a remover:");

                let mut removido = false;

                for i in 0..diretorios.len() {

                    if diretorios[i].nome == nome {

                        diretorios.remove(i);

                        println!("Diretorio removido");

                        removido = true;

                        break;

                    }

                }

                if !removido {

                    println!("Diretorio nao encontrado.");

                }

            },



            11 => {

                let nome = ler_input("Nome do usuario a remover:");

                let mut removido = false;

                let mut uid_removido: u16 = 0;

                for i in 0..usuarios.len() {

                    if usuarios[i].nome == nome {

                        uid_removido = usuarios[i].uid;

                        usuarios.remove(i);

                        removido = true;

                        break;

                    }

                }



                if !removido {

                    println!("Usuario nao encontrado.");

                    continue;

                }



                for i in 0..grupos.len() {

                    grupos[i].remove_membro(uid_removido);

                }



                println!("Lembrete: Usuario removido de grupos tambem");

            },



            12 => {

                let nome = ler_input("Nome do grupo a remover:");

                let mut removido = false;

                for i in 0..grupos.len() {

                    if grupos[i].nome == nome {

                        grupos.remove(i);

                        println!("Grupo removido!");

                        removido = true;

                        break;

                    }

                }

                if !removido {

                    println!("Grupo nao encontrado.");

                }

            },



            13 => {

                let nome = ler_input("Nome do arquivo:");



                let mut posicao_encontrada: usize = arquivos.len();



                for i in 0..arquivos.len() {

                    if arquivos[i].nome == nome {

                        posicao_encontrada = i;

                        break;

                    }

                }



                if posicao_encontrada == arquivos.len() {

                    println!("Arquivo nao encontrado.");

                    continue;

                }



                let dono_leitura  = ler_input("Dono: leitura? apenas s/n")  == "s";

                let dono_escrita  = ler_input("Dono: escrita? Apenas s/n")  == "s";

                let dono_execucao = ler_input("Dono: execucao? Apenas s/n") == "s";

                let permissao_dono = Permissao::new(dono_leitura, dono_escrita, dono_execucao);

                let grupo_leitura  = ler_input("Grupo: leitura? Apenas s/n")  == "s";

                let grupo_escrita  = ler_input("Grupo: escrita? Apenas s/n")  == "s";

                let grupo_execucao = ler_input("Grupo: execucao?Apenas s/n") == "s";

                let permissao_grupo = Permissao::new(grupo_leitura, grupo_escrita, grupo_execucao);

                let outros_leitura  = ler_input("Outros: leitura? Apenas s/n")  == "s";

                let outros_escrita  = ler_input("Outros: escrita? (Apenas s/n ")  == "s";

                let outros_execucao = ler_input("Outros: execucao? Apenas s/n ") == "s";

                let permissao_outros = Permissao::new(outros_leitura, outros_escrita, outros_execucao);



                arquivos[posicao_encontrada]

                    .alterar_permissao((permissao_dono, permissao_grupo, permissao_outros));



                println!("Permissao alterada com sucesso");

            },



            14 => {

                let nome = ler_input("Nome do arquivo:");



                let mut posicao_encontrada: usize = arquivos.len();



                for i in 0..arquivos.len() {

                    if arquivos[i].nome == nome {

                        posicao_encontrada = i;

                        break;

                    }

                }



                if posicao_encontrada == arquivos.len() {

                    println!("Arquivo nao encontrado.");

                    continue;

                }



                arquivos[posicao_encontrada].stat();

            },



            15 => {

                let nome_grupo = ler_input("Nome do grupo:");

                let nome_usuario = ler_input("Nome do usuario:");



                let mut posicao_grupo: usize = grupos.len();

                for i in 0..grupos.len() {

                    if grupos[i].nome == nome_grupo {

                        posicao_grupo = i;

                        break;

                    }

                }

                if posicao_grupo == grupos.len() {

                    println!("Grupo nao encontrado.");

                    continue;

                }



                let mut posicao_usuario: usize = usuarios.len();

                for i in 0..usuarios.len() {

                    if usuarios[i].nome == nome_usuario {

                        posicao_usuario = i;

                        break;

                    }

                }

                if posicao_usuario == usuarios.len() {

                    println!("Usuario nao encontrado.");

                    continue;

                }



                let uid = usuarios[posicao_usuario].uid;

                let gid = grupos[posicao_grupo].gid;

                grupos[posicao_grupo].adiciona_membro(uid);

                usuarios[posicao_usuario].adiciona_grupo(gid);

                println!("Usuario {} adicionado ao grupo {} (uid={})", nome_usuario, nome_grupo, uid);

            },



            16 => {

                let nome = ler_input("Nome do grupo:");



                let mut posicao_encontrada: usize = grupos.len();

                for i in 0..grupos.len() {

                    if grupos[i].nome == nome {

                        posicao_encontrada = i;

                        break;

                    }

                }



                if posicao_encontrada == grupos.len() {

                    println!("Grupo nao encontrado.");

                    continue;

                }



                println!("Membros do grupo {}:", grupos[posicao_encontrada].nome);

                grupos[posicao_encontrada].listar_membros();

            },



            17 => {

                let nome_grupo = ler_input("Nome do grupo:");

                let nome_usuario = ler_input("Nome do usuario:");



                let mut posicao_grupo: usize = grupos.len();

                for i in 0..grupos.len() {

                    if grupos[i].nome == nome_grupo {

                        posicao_grupo = i;

                        break;

                    }

                }

                if posicao_grupo == grupos.len() {

                    println!("Grupo nao encontrado.");

                    continue;

                }



                let mut posicao_usuario: usize = usuarios.len();

                for i in 0..usuarios.len() {

                    if usuarios[i].nome == nome_usuario {

                        posicao_usuario = i;

                        break;

                    }

                }

                if posicao_usuario == usuarios.len() {

                    println!("Usuario nao encontrado.");

                    continue;

                }



                let uid = usuarios[posicao_usuario].uid;

                let gid = grupos[posicao_grupo].gid;

                grupos[posicao_grupo].remove_membro(uid);

                usuarios[posicao_usuario].remove_grupo(gid);

            },



            18 => {

                let nome = ler_input("Nome do usuario:");



                let mut posicao_encontrada: usize = usuarios.len();

                for i in 0..usuarios.len() {

                    if usuarios[i].nome == nome {

                        posicao_encontrada = i;

                        break;

                    }

                }



                if posicao_encontrada == usuarios.len() {

                    println!("Usuario nao encontrado.");

                    continue;

                }



                println!("Grupos do usuario {}:", usuarios[posicao_encontrada].nome);

                usuarios[posicao_encontrada].listar_grupos();

            },



            0 => {

                println!("Saindo...");

                sleep(Duration::from_secs(1));

                break;

            },



            _ => {

                println!("Opcao desconhecida!!");

            },

        }

    }

}



fn main() {

    menu();

}



