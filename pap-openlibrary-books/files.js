import * as fs from "fs";

function writeToFile(name, dataArray) {
  if (!fs.existsSync("files")) {
    fs.mkdirSync("files");
  }

  if (fs.existsSync(`files/${name}.txt`)) {
    fs.rmSync(`files/${name}.txt`, { recursive: true });
    fs.writeFileSync(`files/${name}.txt`, "");
  } else {
    fs.writeFileSync(`files/${name}.txt`, "");
  }

  dataArray.forEach(async (data) => {
    const content = fs.readFileSync(`files/${name}.txt`, "utf-8");
    fs.writeFileSync(`files/${name}.txt`, content + data + "\n");
  });
}

function AppendToFile(name, dataArray) {
  if (!fs.existsSync("files")) {
    fs.mkdirSync("files");
  }

  if (!fs.existsSync(`files/${name}.txt`)) {
    fs.writeFileSync(`files/${name}.txt`, "");
  }

  dataArray.forEach(async (data) => {
    const content = fs.readFileSync(`files/${name}.txt`, "utf-8");
    fs.writeFileSync(`files/${name}.txt`, content + data + "\n");
  });
}

function ReadFromFile(name) {
  const content = fs.readFileSync(`files/${name}.txt`, "utf-8");
  const data = content.split("\n");
  data.pop();
  return data;
}

export { writeToFile, ReadFromFile, AppendToFile };
