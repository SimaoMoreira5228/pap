export interface Livro {
  id: number;
  nome: string;
  resumo?: string;
  n_paginas: number;
  idioma: string;
  img_url?: string;
  ano_edicao?: string;
  autor?: string;
  editora: string;
  id_secao: number;
  categoria?: string;
  sub_categoria?: string;
  requisitado: boolean;
}

export interface permissao {
  id: number;
  acao: string;
  label: string;
}