const fs = require("fs");
const glob = require("glob");
const jsBeautify = require("js-beautify");
const { Command } = require("commander");
const path = require("path");

import generateFormattedCode, { fixTypeSignatureSpacing, fixIndentation } from "./formatters/formatters";

// Format HTML file
function formatHtmlFile(filePath) {
  const htmlContent = fs.readFileSync(filePath, "utf-8");

  const formattedHtml = jsBeautify.html(htmlContent, {
    indent_size: 2,
    wrap_attributes: "force",
  });

  fs.writeFileSync(filePath, formattedHtml);
  console.log(`Formatted HTML: ${filePath}`);
}

// Main formatter function that applies all the formatting steps
function customFormatTs(content, config) {
  // content = replaceDoubleQuotes(content);
  // content = ensureSemicolons(content);
  // content = enforceTwoSpaceIndentation(content);
  // content = fixDeclaration(content);
  // content = formatIfStatements(content);
  // content = formatFunctionalOperators(content);
  // content = formatImportsAndClass(
  //   content,
  //   config.indentation.size
  // );
  //content = indentClassAndMethods(content, config.indentation.size);
  content = fixTypeSignatureSpacing(content);
  content = fixIndentation(content, config);
  const options = {
    indentation: {
      type: 'spaces',
      size: 2,
    },
    trailingComma: false,
    semicolon: true,
    maxPropertiesPerLine: 1,
  };
  
  const options2 = {
    indentation: {
      type: 'spaces',
      size: 4,
    },
    trailingComma: true,
    semicolon: true,
    maxPropertiesPerLine: 3,
  };

  const options3 = {
    indentation: {
      type: 'spaces',
      size: 2,
    },
    trailingComma: false,
    semicolon: false,
    maxPropertiesPerLine: 2,
  };
  
  
  
  
  console.log(generateFormattedCode('const test: {nice: boolean, hello: string} = {nice: true,hello: "yes"};', options));
  console.log(generateFormattedCode('const test: {nice: boolean, hello: string} = {nice: true,hello: "yes"};', options2));
  console.log(generateFormattedCode('const test: {nice: boolean, hello: string} = {nice: true,hello: "yes"};', options3));
  return content;
}

// Format TypeScript file using custom logic
function formatTsFile(filePath, config) {
  const tsContent = fs.readFileSync(filePath, "utf-8");
  const formattedTs = customFormatTs(tsContent, config);
  fs.writeFileSync(filePath, formattedTs);
  console.log(`Custom Formatted TS: ${filePath}`);
}

// Format both HTML and TS files
function formatFiles(pattern) {
  // Load the config.json file
  const configPath = path.resolve(__dirname, "config.json");
  const configData = fs.readFileSync(configPath);

  // Parse the JSON data
  const config = JSON.parse(configData);
  const files = glob.sync(pattern);
  files.forEach((filePath) => {
    if (filePath.endsWith(".html")) {
      formatHtmlFile(filePath);
    } else if (filePath.endsWith(".ts")) {
      formatTsFile(filePath, config);
    }
  });
}

// Command-line interface
if (require.main === module) {
  console.log("start");

  const program = new Command();

  program
    .option(
      "-p, --pattern <pattern>",
      "Glob pattern to match files",
      "**/*.{html,ts}"
    )
    .parse(process.argv);

  const options = program.opts();
  formatFiles(options.pattern);
}
