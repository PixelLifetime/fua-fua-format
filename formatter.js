"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
var glob = require("glob");
var jsBeautify = require("js-beautify");
var Command = require("commander").Command;
var path = require("path");
var indentClassAndMethods = require("./formatters/indentClassAndMethods.js");
var formatFunctionalOperators_ts_1 = require("./formatters/formatFunctionalOperators.ts");
var replaceDoubleQuotes = require("./formatters/replaceDoubleQuotes.js");
var ensureSemicolons = require("./formatters/ensureSemicolons.js");
var enforceTwoSpaceIndentation = require("./formatters/enforceTwoSpaceIndentation.js");
var fixDeclaration = require("./formatters/fixMethodOrVarDeclaration.js");
var formatIfStatements = require("./formatters/formatIfStatements.js");
var formatImportsAndClass = require("./formatters/formatImportsAndClass.js");
// rest of your code...
// Format HTML file
function formatHtmlFile(filePath) {
    var htmlContent = fs.readFileSync(filePath, "utf-8");
    var formattedHtml = jsBeautify.html(htmlContent, {
        indent_size: 2,
        wrap_attributes: "force",
    });
    fs.writeFileSync(filePath, formattedHtml);
    console.log("Formatted HTML: ".concat(filePath));
}
// Main formatter function that applies all the formatting steps
function customFormatTs(content, config) {
    content = replaceDoubleQuotes(content);
    content = ensureSemicolons(content);
    content = enforceTwoSpaceIndentation(content);
    content = fixDeclaration(content);
    content = formatIfStatements(content);
    content = (0, formatFunctionalOperators_ts_1.default)(content);
    content = formatImportsAndClass(content, config.indentation.size);
    content = indentClassAndMethods(content, config.indentation.size);
    return content;
}
// Format TypeScript file using custom logic
function formatTsFile(filePath, config) {
    var tsContent = fs.readFileSync(filePath, "utf-8");
    var formattedTs = customFormatTs(tsContent, config);
    fs.writeFileSync(filePath, formattedTs);
    console.log("Custom Formatted TS: ".concat(filePath));
}
// Format both HTML and TS files
function formatFiles(pattern) {
    // Load the config.json file
    var configPath = path.resolve(__dirname, "config.json");
    var configData = fs.readFileSync(configPath);
    // Parse the JSON data
    var config = JSON.parse(configData);
    var files = glob.sync(pattern);
    files.forEach(function (filePath) {
        if (filePath.endsWith(".html")) {
            formatHtmlFile(filePath);
        }
        else if (filePath.endsWith(".ts")) {
            formatTsFile(filePath, config);
        }
    });
}
// Command-line interface
if (require.main === module) {
    console.log("start");
    var program = new Command();
    program
        .option("-p, --pattern <pattern>", "Glob pattern to match files", "**/*.{html,ts}")
        .parse(process.argv);
    var options = program.opts();
    formatFiles(options.pattern);
}
