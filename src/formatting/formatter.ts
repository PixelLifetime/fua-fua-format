import * as jsBeautify from "js-beautify";
import * as fs from "fs";
import * as glob from "glob";
import * as path from "path";
import { FormatterConfig } from "../../configTypes";
import { Formatters } from "./formatters";

export class Formatter {

  /**
   * Class that contains functions for formatting code
   */
  private _formatters: Formatters = new Formatters();

  /**
   * Formats both HTML and TypeScript files based on the provided pattern.
   * It reads the configuration from the config file, searches for files matching the
   * provided pattern, and formats them based on their extension.
   *
   * @param pattern - Glob pattern to match files in the `filesForFormatting` directory.
   */
  public formatFiles(pattern: string): void {
    const configPath = path.resolve(__dirname, "../../config.json");
    const configData = fs.readFileSync(configPath, "utf-8");

    const config: FormatterConfig = JSON.parse(configData);

    const files = glob.sync(path.resolve(__dirname, "../filesForFormatting").replace(/\\/g, "/") + pattern);

    files.forEach((filePath) => {
      if (filePath.endsWith(".html")) {
        this._readAndFormatHtmlFile(filePath);
      } else if (filePath.endsWith(".ts")) {
        this._readAndFormatTsFile(filePath, config);
      }
    });
  }

  /**
   * Main formatter function that applies all the formatting steps to TypeScript content.
   * This includes fixing type signature spacing, applying indentation, and replacing double quotes.
   *
   * @param content - The TypeScript content to format.
   * @param config - The configuration object specifying formatting rules.
   * @returns The formatted TypeScript content as a string.
   */
  private _customFormatTs(content: string, config: FormatterConfig): string {
    content = this._formatters.fixTypeSignatureSpacing(content);
    content = this._formatters.fixIndentation(content, config);
    content = this._formatters.replaceDoubleQuotes(content);
    return content;
  }

  /**
   * Reads and formats a TypeScript file using custom formatting logic.
   * It reads the file, applies formatting rules, and writes the formatted content back to the file.
   *
   * @param filePath - The path of the TypeScript file to format.
   * @param config - The configuration object specifying formatting rules.
   */
  private _readAndFormatTsFile(filePath: string, config: FormatterConfig): void {
    const tsContent = fs.readFileSync(filePath, "utf-8");
    const formattedTs = this._customFormatTs(tsContent, config);
    fs.writeFileSync(filePath, formattedTs);
    console.log(`Custom Formatted TS: ${filePath}`);
  }
  
  /**
   * Reads and formats an HTML file using `js-beautify`.
   * It reads the file, applies HTML formatting rules (e.g., indentation and attribute wrapping),
   * and writes the formatted content back to the file.
   *
   * @param filePath - The path of the HTML file to format.
   */
  private _readAndFormatHtmlFile(filePath: string): void {
    const htmlContent = fs.readFileSync(filePath, "utf-8");

    const formattedHtml = jsBeautify.html(htmlContent, {
      indent_size: 2,
      wrap_attributes: "force",
    });

    fs.writeFileSync(filePath, formattedHtml);
    console.log(`Formatted HTML: ${filePath}`);
  }
}
