import { FormatterConfig } from "../../../configTypes";

export class FormattedCodeGenerators {
  /**
   * Function that generates a properly formatted object declaration with type and assignment.
   * For example, the input:
   * const test: {nice: boolean, hello: string} = { nice: true, hello: "yes"};
   *
   * will be formatted as:
   * const test: {
   *   nice: boolean,
   *   hello: string
   * } = {
   *   nice: true,
   *   hello: "yes"
   * };
   *
   * @param code - The input code to be formatted, e.g., const test: {nice: boolean, hello: string} = { nice: true, hello: "yes"};
   * @param config - The formatting configuration.
   * @returns The formatted code as shown in the example.
   */
  public formattedObjectTypeAndAssignment(
    code: string,
    config: FormatterConfig
  ): string {
    const indentSize = config.indentation.size;
    const indentType = config.indentation.type;
    const indentString =
      indentType === "tabs" ? "\t".repeat(indentSize) : " ".repeat(indentSize);

    // Regular expression to match the variable declaration
    const variableDeclarationRegex =
      /(const|let|var)\s+(\w+)(\s*:\s*\{[^}]+\})?\s*=\s*(\{[^}]+\});?/;

    const match = code.match(variableDeclarationRegex);

    if (!match) {
      return code;
    }

    const [fullMatch, varKeyword, varName, typeAnnotation, initializer] = match;

    let formattedTypeAnnotation = "";
    if (typeAnnotation) {
      const typeContent = typeAnnotation.slice(
        typeAnnotation.indexOf("{") + 1,
        typeAnnotation.lastIndexOf("}")
      );
      const typeProperties = typeContent
        .split(/\s*[,;]\s*/)
        .filter(Boolean)
        .map((prop) => {
          return prop.trim();
        });

      const useMultiline =
        typeProperties.length > config.objectFormatting.maxPropertiesPerLine;

      if (useMultiline) {
        formattedTypeAnnotation = `: {\n`;
        typeProperties.forEach((prop, index) => {
          const semicolon =
            config.typeFormatting.trailingSemicolon ||
            index < typeProperties.length - 1
              ? ";"
              : "";
          formattedTypeAnnotation += `${indentString}${prop}${semicolon}\n`;
        });
        formattedTypeAnnotation += `}`;
      } else {
        const semicolon = config.typeFormatting.trailingSemicolon ? ";" : "";
        formattedTypeAnnotation = `: { ${typeProperties.join(
          "; "
        )}${semicolon} }`;
      }
    }

    const initContent = initializer.slice(
      initializer.indexOf("{") + 1,
      initializer.lastIndexOf("}")
    );
    const initProperties = initContent
      .split(/\s*,\s*/)
      .filter(Boolean)
      .map((prop) => {
        return prop.trim();
      });

    const useMultilineInit =
      initProperties.length > config.objectFormatting.maxPropertiesPerLine;

    let formattedInitializer = "";
    if (useMultilineInit) {
      formattedInitializer = `= {\n`;
      initProperties.forEach((prop, index) => {
        const comma =
          config.objectFormatting.trailingComma ||
          index < initProperties.length - 1
            ? ","
            : "";
        formattedInitializer += `${indentString}${prop}${comma}\n`;
      });
      formattedInitializer += `}`;
    } else {
      const comma = config.objectFormatting.trailingComma ? "," : "";
      formattedInitializer = `= { ${initProperties.join(", ")}${comma} }`;
    }

    const formattedCode = `${varKeyword} ${varName}${formattedTypeAnnotation} ${formattedInitializer};`;

    return formattedCode;
  }

  /**
   * Function that generates a properly formatted array.
   * For example, the input:
   * const testArray: string[] = ["hello", "my", "dear", "friend"];
   *
   * will be formatted as:
   * const testArray: string[] = [
   *   "hello",
   *   "my",
   *   "dear",
   *   "friend"
   * ];
   *
   * @param code - The input code to be formatted, e.g., const testArray: string[] = ["hello", "my", "dear", "friend"];
   * @param config - The formatting configuration.
   * @returns The formatted code as shown in the example.
   */
  public formattedArray(code: string, config: FormatterConfig): string {
    const indentSize = config.indentation.size;
    const indentType = config.indentation.type;
    const indentString =
      indentType === "tabs" ? "\t".repeat(indentSize) : " ".repeat(indentSize);

    // Regular expression to match the array declaration
    const arrayRegex = /^([ \t]*)([\s\S][^\n/]*)\[([^/(\]]*)\]/;

    const match = code.match(arrayRegex);

    if (!match) {
      return code;
    }

    const [fullMatch, indentation, line, elementsStr] = match;
    // console.log([fullMatch, indentation, line, elementsStr]);
    const elements = elementsStr
      .split(/\s*,\s*/)
      .filter(Boolean)
      .map((prop) => {
        return prop.trim();
      });

    const useMultilineInit =
      elements.length > config.arrayFormatting.maxElementsPerLine;

    let formattedArray = "";
    if (useMultilineInit) {
      formattedArray = `[\n`;
      elements.forEach((prop, index) => {
        const comma =
          index < elements.length - 1
            ? ","
            : "";
        formattedArray += `${indentString}${prop}${comma}\n`;
      });
      formattedArray += `]`;
    } else {
      formattedArray = `[${elements.join(", ")}]`;
    }

    const formattedCode = `${indentation}${line}${formattedArray};`;

    return formattedCode;
  }

  /**
   * Function that generates a properly formatted inputs block
   * For example, the input:
   * {   Component, inject, OnInit, PLATFORM_ID,ViewChild} from '@angular/core';
   *
   * will be formatted as:
   * { Component, inject, OnInit, PLATFORM_ID, ViewChild } from '@angular/core';
   *
   * @param code - The input code to be formatted, e.g., import {   Component, inject, OnInit, PLATFORM_ID,ViewChild} from '@angular/core';
   * @param config - The formatting configuration.
   * @returns The formatted code as shown in the example.
   */
  public formattedImports(code: string, config: FormatterConfig): string {
    const importRegex =
      /import\s*{?\s*([^}]*?)\s*}?\s*from\s*['"]([^'"]+)['"](;)?/g;

    const match = code.match(importRegex);

    if (!match) {
      return code;
    }

    const formattedCode = code.replace(importRegex, (_, imports, fromPath) => {
      const formattedImports = imports
        .split(",")
        .map((imp) => imp.trim())
        .join(", ");
      return config.importFormat.spacesAroundImports
        ? `import { ${formattedImports} } from '${fromPath}';`
        : `import {${formattedImports}} from '${fromPath}';`;
    });

    return formattedCode;
  }
}
