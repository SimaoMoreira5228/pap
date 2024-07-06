use sqlx::{MySql, Pool};

pub async fn create_tables(pool: &Pool<MySql>) -> Result<(), String> {
    let authors = "CREATE TABLE IF NOT EXISTS `autores` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  `nacionalidade` varchar(255) DEFAULT NULL,
  `data_nasc` varchar(255) DEFAULT NULL,
  `data_morte` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2110 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let categories = "CREATE TABLE IF NOT EXISTS `categorias` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=24 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let publishers = "CREATE TABLE IF NOT EXISTS `editoras` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `morada` varchar(255) DEFAULT NULL,
  `codigo_postal` varchar(255) DEFAULT NULL,
  `telefone` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=414 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let readers = "CREATE TABLE IF NOT EXISTS `leitores` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  `morada` varchar(255) NOT NULL,
  `telefone` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let books = "CREATE TABLE IF NOT EXISTS `livros` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  `resumo` longtext DEFAULT NULL,
  `n_paginas` int(11) NOT NULL,
  `idioma` varchar(255) NOT NULL,
  `img_url` text DEFAULT NULL,
  `ano_edicao` varchar(255) DEFAULT NULL,
  `id_autor` int(11) DEFAULT NULL,
  `id_editora` int(11) NOT NULL,
  `id_secao` int(11) NOT NULL,
  `id_sub_categoria` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `livros_id_secao` (`id_secao`),
  KEY `livros_id_autor` (`id_autor`),
  KEY `livros_id_editora` (`id_editora`),
  KEY `livros_ibfk_5` (`id_sub_categoria`),
  CONSTRAINT `livros_ibfk_2` FOREIGN KEY (`id_secao`) REFERENCES `secoes` (`id`),
  CONSTRAINT `livros_ibfk_3` FOREIGN KEY (`id_autor`) REFERENCES `autores` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `livros_ibfk_4` FOREIGN KEY (`id_editora`) REFERENCES `editoras` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `livros_ibfk_5` FOREIGN KEY (`id_sub_categoria`) REFERENCES `sub_categorias` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2921 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let permissions = "CREATE TABLE IF NOT EXISTS `permissoes` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `acao` varchar(255) NOT NULL,
  `label` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=29 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let permissions_insert = "INSERT INTO `permissoes` (id, acao, label) VALUES
(1,'criar_livro','Criar Livro'),
(2,'atualizar_livro','Atualizar Livro'),
(3,'apagar_livro','Apagar Livro'),
(4,'criar_bibliotecario','Criar Bibliotecário'),
(5,'atualizar_bibliotecario','Atualizar Bibliotecário'),
(6,'apagar_bibliotecario','Apagar Bibliotecário'),
(7,'criar_leitor','Criar Leitor'),
(8,'atualizar_leitor','Atualizar Leitor'),
(9,'apagar_leitor','Apagar Leitor'),
(10,'criar_autor','Criar Autor'),
(11,'atualizar_autor','Atualizar Autor'),
(12,'apagar_autor','Apagar Autor'),
(13,'criar_cargo','Criar Cargo'),
(14,'atualizar_cargo','Atualizar Cargo'),
(15,'apagar_cargo','Apagar Cargo'),
(16,'criar_categoria','Criar Categoria'),
(17,'atualizar_categoria','Atualizar Categoria'),
(18,'apagar_categoria','Apagar Categoria'),
(19,'criar_requisicao','Criar Requisição'),
(20,'atualizar_requisicao','Atualizar Requisição'),
(21,'apagar_requisicao','Apagar Requisição'),
(22,'criar_secao','Criar Seção'),
(23,'atualizar_secao','Atualizar Seção'),
(24,'apagar_secao','Apagar Seção'),
(25,'criar_sub_categoria','Criar Subcategoria'),
(26,'atualizar_sub_categoria','Atualizar Subcategoria'),
(27,'apagar_sub_categoria','Apagar Subcategoria'),
(28,'mudar_configuracoes','Mudar Configurações')
ON DUPLICATE KEY UPDATE id=VALUES(id);"; // Avoid duplicates in case of re-run

    let cargos = "CREATE TABLE IF NOT EXISTS `cargos` (
  `nome` varchar(255) NOT NULL,
  `permissao` int(11) NOT NULL,
  KEY `cargos_permissoes_FK` (`permissao`),
  CONSTRAINT `cargos_permissoes_FK` FOREIGN KEY (`permissao`) REFERENCES `permissoes` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let requests = "CREATE TABLE IF NOT EXISTS `requisicoes` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `id_leitor` int(11) NOT NULL,
  `id_livro_requisitado` int(11) NOT NULL,
  `data_requisicao` datetime NOT NULL,
  `data_entrega` datetime DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `requisicoes_id_leitor` (`id_leitor`),
  KEY `requisicoes_id_livro_requisitado` (`id_livro_requisitado`),
  CONSTRAINT `requisicoes_ibfk_1` FOREIGN KEY (`id_leitor`) REFERENCES `leitores` (`id`),
  CONSTRAINT `requisicoes_ibfk_2` FOREIGN KEY (`id_livro_requisitado`) REFERENCES `livros` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let sections = "CREATE TABLE IF NOT EXISTS `secoes` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `id_categoria` int(11) NOT NULL,
  `nome` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `seccoes_id_categoria` (`id_categoria`),
  CONSTRAINT `secoes_ibfk_1` FOREIGN KEY (`id_categoria`) REFERENCES `categorias` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let sub_categories = "CREATE TABLE IF NOT EXISTS `sub_categorias` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `id_categoria` int(11) NOT NULL,
  `nome` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `FK_sub_categoria_categorias` (`id_categoria`),
  CONSTRAINT `FK_sub_categoria_categorias` FOREIGN KEY (`id_categoria`) REFERENCES `categorias` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=331 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    let librarians = "CREATE TABLE IF NOT EXISTS `bibliotecarios` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `cargo` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;";

    sqlx::query(permissions).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela permissoes: {}", e);
        format!("Falha ao criar tabela permissoes: {}", e)
    })?;

    sqlx::query(permissions_insert)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao inserir dados na tabela permissoes: {}", e);
            format!("Falha ao inserir dados na tabela permissoes: {}", e)
        })?;

    sqlx::query(authors).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela autores: {}", e);
        format!("Falha ao criar tabela autores: {}", e)
    })?;

    sqlx::query(cargos).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela cargos: {}", e);
        format!("Falha ao criar tabela cargos: {}", e)
    })?;

    sqlx::query(librarians).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela bibliotecarios: {}", e);
        format!("Falha ao criar tabela bibliotecarios: {}", e)
    })?;

    sqlx::query(categories).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela categorias: {}", e);
        format!("Falha ao criar tabela categorias: {}", e)
    })?;

    sqlx::query(sub_categories)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao criar tabela sub_categorias: {}", e);
            format!("Falha ao criar tabela sub_categorias: {}", e)
        })?;

    sqlx::query(sections).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela seccoes: {}", e);
        format!("Falha ao criar tabela seccoes: {}", e)
    })?;

    sqlx::query(publishers).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela editoras: {}", e);
        format!("Falha ao criar tabela editoras: {}", e)
    })?;

    sqlx::query(readers).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela leitores: {}", e);
        format!("Falha ao criar tabela leitores: {}", e)
    })?;

    sqlx::query(books).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela livros: {}", e);
        format!("Falha ao criar tabela livros: {}", e)
    })?;

    sqlx::query(requests).execute(pool).await.map_err(|e| {
        tracing::error!("Falha ao criar tabela requisicoes: {}", e);
        format!("Falha ao criar tabela requisicoes: {}", e)
    })?;

    Ok(())
}
