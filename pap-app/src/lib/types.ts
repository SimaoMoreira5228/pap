export interface Livro {
  id: number;
  nome: string;
  resumo?: string;
  n_paginas: number;
  idioma: string;
  img_url?: string;
  ano_edicao?: string;
  autor?: string;
  autor_id?: number;
  editora: string;
  id_secao: number;
  categoria?: string;
  sub_categoria?: string;
  requisitado: boolean;
}

export interface bibliotecario {
  id: number;
  nome: string;
  cargo: string;
}

export interface Leitor {
  id: string;
  nome: string;
  morada: string;
  telefone: number;
  email: string;
}

export interface Categoria {
  id: number;
  nome: string;
}

export interface Secao {
  id: number;
  id_categoria: number;
  nome: string;
}

export interface SubCategoria {
  id: number;
  id_categoria: number;
  nome: string;
}

export interface permissao {
  id: number;
  acao: string;
  label: string;
}

export interface Autor {
  id: number;
  nome: string;
  nacionalidade?: string;
  data_nasc?: string;
  data_morte?: string;
}

export interface Requisicao {
  id: number;
  id_leitor: number;
  id_livro_requisitado: number;
  data_requisicao: string;
  data_entrega?: string;
}

export interface Editora {
  id: number;
  nome: string;
  morada?: string;
  codigo_postal?: string;
  telefone?: string;
  email?: string;
}

export interface Cargo {
  nome: string;
  permissao: number;
}

export enum DatabaseConnectionStatus {
  NOT_CONNECTED,
  CONNECTING,
  CONNECTED,
  BROKEN,
}
