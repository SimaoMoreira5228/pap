import { connect } from "puppeteer-real-browser";
import { writeToFile, AppendToFile, ReadFromFile } from "./files.js";
import {
  CategoriesUrls,
  subCategoriesUlIds,
  subCategoriesUrls,
} from "./arrays.js";
import mysql from "mysql2/promise";
import { exit } from "process";

let instances = 0;

const pool = mysql.createPool({
  host: "localhost",
  user: "root",
  database: "libra_hub",
  waitForConnections: true,
  connectionLimit: 10,
});

class Book {
  title;
  imgUrl;
  resume;
  pages;
  idioma;
  autor;
  editora;
  anoEdicao;
  subCategoria;

  id_autor;
  id_editora;
  id_categoria;
  id_sub_categoria;

  constructor(
    title,
    imgUrl,
    resume,
    pages,
    idioma,
    autor,
    editora,
    anoEdicao,
    subCategoria
  ) {
    this.title = title;
    this.imgUrl = imgUrl;
    this.resume = resume;
    this.pages = pages;
    this.idioma = idioma;
    this.autor = autor;
    this.editora = editora;
    this.anoEdicao = anoEdicao;
    this.subCategoria = subCategoria;
  }

  async setCategoriesIds() {
    const [rows, fields] = await pool.query(
      "SELECT id, id_categoria FROM sub_categorias WHERE nome = ?",
      [this.subCategoria]
    );

    this.id_categoria = rows[0].id_categoria;
    this.id_sub_categoria = rows[0].id;
  }

  async setAuthorId() {
    if (!this.autor) {
      this.autor = "Desconhecido";
    }

    const [rows, fields] = await pool.query(
      "SELECT id FROM autores WHERE nome = ?",
      [this.autor]
    );

    if (rows.length === 0) {
      await pool.query("INSERT INTO autores (nome) VALUES (?)", [this.autor]);
    } else {
      this.id_autor = rows[0].id;
      return;
    }

    const [rows2, fields2] = await pool.query(
      "SELECT id FROM autores WHERE nome = ?",
      [this.autor]
    );

    this.id_autor = rows2[0].id;
  }

  async setEditoraId() {
    if (!this.editora) {
      this.editora = "Desconhecido";
    }

    const [rows, fields] = await pool.query(
      "SELECT id FROM editoras WHERE nome = ?",
      [this.editora]
    );

    if (rows.length === 0) {
      await pool.query("INSERT INTO editoras (nome) VALUES (?)", [
        this.editora,
      ]);
    } else {
      this.id_editora = rows[0].id;
      return;
    }

    const [rows2, fields2] = await pool.query(
      "SELECT id FROM editoras WHERE nome = ?",
      [this.editora]
    );

    this.id_editora = rows2[0].id;
  }

  async initialize() {
    await this.setAuthorId();
    await this.setCategoriesIds();
    await this.setEditoraId();
  }

  async save() {
    await this.initialize();

    await pool.query(
      "INSERT INTO livros (nome, resumo, n_paginas, idioma, img_url, ano_edicao, id_autor, id_editora, id_sub_categoria) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
      [
        this.title,
        this.resume,
        this.pages,
        this.idioma,
        this.imgUrl,
        this.anoEdicao,
        this.id_autor,
        this.id_editora,
        this.id_sub_categoria,
      ]
    );
  }
}

async function getSubCategoriesUrls(save = false) {
  const subCategories = await Promise.all(
    CategoriesUrls.map(async (link, index) => {
      while (instances >= 3) {
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      instances++;

      const { page } = await connect({
        headless: "auto",
        fingerprint: true,
        turnstile: true,
        tf: true,
      });

      await page.goto(link, {
        waitUntil: "domcontentloaded",
      });

      const subCategoriesUl = await page.$(`#${subCategoriesUlIds[index]}`);
      const subCategories = await subCategoriesUl.$$eval(
        "li",
        (lis, index) =>
          lis.map((li) => {
            const subCategory = li.querySelector("a").href;
            const subCategoryName = li.querySelector("a").textContent;
            return { index, subCategory, subCategoryName };
          }),
        index
      );

      await page.close();
      instances--;

      return subCategories;
    })
  );

  if (save) {
    await Promise.all(subCategories.flat().map(async (subCategory) => {
      const [rows, fields] = await pool.query(
        "SELECT id FROM sub_categorias WHERE nome = ?",
        [subCategory.subCategoryName]
      );

      if (rows.length === 0) {
        await pool.query(
          "INSERT INTO sub_categorias (id_categoria, nome) VALUES (?, ?)",
          [subCategory.index + 1, subCategory.subCategoryName]
        );
      }
    }));
  }

  writeToFile(
    "subCategories",
    subCategories.flat().map((subCategory) => subCategory.subCategory)
  );
}

async function getBooksUrls() {
  await Promise.all(
    subCategoriesUrls.map(async (link) => {
      while (instances >= 5) {
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      instances++;

      const { page } = await connect({
        headless: "auto",
        fingerprint: true,
        turnstile: true,
        tf: true,
      });

      await page.goto(link, {
        waitUntil: "domcontentloaded",
      });

      console.log(`Getting books from ${link}`);

      const bookLimit = 10;

      const searchDiv = await page.$("div > .search-products");

      if (!searchDiv) {
        console.log("No search div found");
        await page.close();
        instances--;
        return;
      }

      const booksPortlets = await searchDiv.$$("div > .product-portlet");

      if (!booksPortlets) {
        console.log("No books portlets found");
        await page.close();
        instances--;
        return;
      }

      const books = [];
      for (let index = 0; index < booksPortlets.length; index++) {
        const bookPortlet = booksPortlets[index];
        const book = await bookPortlet.$eval(
          "div > .product-info > div > div > div > div > p > a",
          (a) => {
            if (a) {
              return a.href;
            }
          }
        );

        if (index < bookLimit) {
          books.push(book);
        } else {
          break;
        }
      }

      // console.log(`Got ${books.length} books from ${link}`);

      AppendToFile("books", books);
      await page.close();
      instances--;
    })
  );
}

async function getBooksInfo() {
  const booksArray = ReadFromFile("books");

  await Promise.all(
    booksArray.map(async (link) => {
      while (instances >= 6) {
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      instances++;

      const { page } = await connect({});

      try {
        const response = await page.goto(link, {
          waitUntil: "domcontentloaded",
        });

        const imgUrl = await page.$eval("img.lazyload", (img) => img.src);

        const title = await page.$eval(
          "#productPageSectionDetails-collapseDetalhes-content-title",
          (div) => div.textContent
        );

        const authorDiv = await page.$(
          "#productPageSectionDetails-collapseDetalhes-content-author"
        );

        let author = "Desconhecido";
        try {
          const authorA = await authorDiv.$("a");
          if (authorA) {
            author = await authorA.evaluate((a) => a.textContent);
          }

          if (author === "Desconhecido") {
            author = await authorDiv.evaluate((div) =>
              div.textContent.replace("de ", "").trim()
            );
          }
        } catch (error) {
          console.log("no author found " + link);
        }

        if (author === "Desconhecido") {
          console.log("\n" + link + "\n" + author + "\n");
        }

        const anoEdicao = await page.$eval(
          "#productPageSectionDetails-collapseDetalhes-content-year",
          (div) => div.textContent
        );

        const allDivs = await page.$$("div.col-xs-12 > div.info");
        const editora = await allDivs[2].evaluate((div) => div.textContent);

        const idioma = await page.$eval(
          "#productPageSectionDetails-collapseDetalhes-content-language",
          (div) => div.textContent
        );

        let pages = "0";
        try {
          const pagesDiv = await page.$(
            "#productPageSectionDetails-collapseDetalhes-content-nrPages"
          );

          if (pagesDiv) {
            pages = await pagesDiv.evaluate((div) => div.textContent);
          }
        } catch (error) {
          console.log("no pages found " + link);
        }

        const subCategoriaDiv = await page.$(
          "#productPageSectionDetails-collapseDetalhes-content-themes"
        );

        const subCategorias = await subCategoriaDiv.$$("a");
        const subCategoria = await subCategorias[
          subCategorias.length - 1
        ].evaluate((a) => a.textContent);

        const resumeDiv = await page.$("#productPageSectionAboutBook-sinopse");

        let resume = "Sem resumo";
        try {
          const resumeP = await resumeDiv.$("p");
          if (resumeP) {
            resume = await resumeP.evaluate((p) => p.textContent);
          }
        } catch (error) {
          console.log("no resume found " + link);
        }

        const book = new Book(
          title.trim(),
          imgUrl,
          resume.trim(),
          pages.replace("Páginas:", "").trim(),
          idioma.replace("Idioma:", "").trim(),
          author.trim(),
          editora.trim(),
          anoEdicao.replace("Ano de edição:", "").trim(),
          subCategoria.trim()
        );

        await book.save();
      } catch (error) {
        console.log("Error fetching book link: " + link + "\n", error);
      } finally {
        await page.close();
        instances--;
      }
    })
  );
}

(async () => {
  await getBooksInfo();
})();
