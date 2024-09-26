import { FormatterConfig } from "../../configTypes";

export class Formatters {
  /**
   * Function that fixes spacing around type signatures. It converts:
   *
   * `variable : SomeType`
   *
   * to:
   *
   * `variable: SomeType`
   *
   * by ensuring there is exactly one space after the colon.
   *
   * @param content - The content string where type signatures need to be fixed.
   * @returns The formatted string with corrected spacing around type signatures.
   */
  public fixTypeSignatureSpacing(content: string): string {
    return content.replace(/\s*:\s*/g, ": ");
  }

  /**
   * Function that formats the given content by adjusting indentation and spacing around
   * braces and parentheses. It processes the entire file, adding new lines, indentation,
   * and removing unnecessary spaces based on configuration settings.
   *
   * It also applies regex rules to check for specific patterns in the code (e.g., function
   * declarations, control structures) and formats them accordingly.
   *
   * @param content - The content string (usually code) that needs to be formatted.
   * @param config - The configuration object that dictates the formatting rules (e.g., indentation size, type, object formatting).
   * @returns The formatted content string with proper indentation, new lines, and spacing.
   */
  public fixIndentation(content: string, config: FormatterConfig): string {
    const lines = content.split("\n");
    const indentedLines: string[] = [];
    let currentIndentLevel = 0;
    const indentSize = config.indentation.size;
    const indentType = config.indentation.type;
    const indentString =
      indentType === "tabs" ? "\t".repeat(indentSize) : " ".repeat(indentSize);
    const functionDeclarationRegex = /(\w+\([^)]*\)\s*)(\s*:\s*[^{]*)?{/;
    const controlStructuresRegex = /\w+\s+\(([^,)]+)\)\s+\{/;
    const squareBracesRegex = /^([ \t]*)([\s\S][^\n/]*)\[([^/(\]]*)\]/gm;
    // const roundBracesRegex =
    //   /^([ \t]*)([\s\S][^\n/]*)\(([^/(\]]*)\)/gm;
    const roundBracesRegex =
    /\(\s*([^,()]+)?(?:\s*,\s*([^,()]+))?(?:\s*,\s*([^,()]+))?\s*\)/g;
    const curlyBracesRegex =
      /\{\s*([^,;{}]+)?(?:\s*,\s*([^,;{}]+))?(?:\s*,\s*([^,;{}]+))?\s*\}/g;

    const objectAssignRegex =
      /([ \t]*)(\b(?:const|let|var)\s+\w+(?:\s*:\s*(?:\{[\s\S]*?\}|\w+))?\s*=\s*)\{\s*((?:[^{}]*?(?:,)?\s*)+)\}/gm;
    const typeDefinitionRegex =
      /^([ \t]*)(\b(?:const|let|var)\s+\w+)\s*:\s*(\{[\s\S]*?\})(.*$)/gm;

    const countChildrenInObject = /\{\s*((?:[^{}]*?(?:,)\s*){3,})\}/;

    const interpolationRegex = /\$\{(\s*.*\s*)\}/g;
    const importRegex =
      /import\s*{?\s*([^}]*?)\s*}?\s*from\s*['"]([^'"]+)['"](;)?/g;

    for (let line of lines) {
      line = line.trim(); // Trim the line
      let resultLine = indentString.repeat(currentIndentLevel); // Start with current indentation
      let i = 0;

      // Skip empty lines if desired
      if (line === "") {
        indentedLines.push("");
        continue;
      }

      // Corrected comment detection
      if (/^\s*(\/\/|\/\*|\*|\*\/)/.test(line)) {
        indentedLines.push(resultLine + line);
        continue;
      }

      // Handle function declarations
      if (functionDeclarationRegex.test(line)) {
        indentedLines.push(resultLine + line);
        currentIndentLevel++;
        continue;
      }

      // Handle control structures
      if (controlStructuresRegex.test(line)) {
        indentedLines.push(resultLine + line);
        currentIndentLevel++;
        continue;
      }

      // Process each character in the line
      while (i < line.length) {
        const char = line[i];

        if (char === "{" || char === "(" || char === "[") {
          // Add the character to resultLine
          resultLine += char;
          i++;

          // Increase indent level
          currentIndentLevel++;

          // Check if there's already a newline after the opening brace
          if (i < line.length && (line[i] === "\n" || line[i] === "\r")) {
            // There's already a newline, so we don't need to add one
            while (i < line.length && (line[i] === "\n" || line[i] === "\r")) {
              // Add existing newline characters to resultLine
              resultLine += line[i];
              i++;
            }
            // Start a new line with increased indentation
            resultLine += indentString.repeat(currentIndentLevel);
          } else {
            // No newline present, so we add one
            // Push the current line to indentedLines
            if (resultLine.trim() !== "") {
              indentedLines.push(resultLine);
            }
            // Start a new line with increased indentation
            resultLine = indentString.repeat(currentIndentLevel);
          }
        } else if (char === "}" || char === ")" || char === "]") {
          // Push the current line to indentedLines if not empty
          if (resultLine.trim() !== "") {
            indentedLines.push(resultLine);
          }

          // Decrease indent level
          currentIndentLevel = Math.max(currentIndentLevel - 1, 0);

          // Start a new line with decreased indentation
          resultLine = indentString.repeat(currentIndentLevel);

          // Add the closing character
          resultLine += char;
          i++;
        } else {
          // Add the character to resultLine
          resultLine += char;
          i++;
        }
      }

      // After processing the line, add the resultLine to indentedLines if not empty
      if (resultLine.trim() !== "") {
        indentedLines.push(resultLine);
      }
    }

    let completedIndentedLines = indentedLines.join("\n");

    // Format square braces
    completedIndentedLines = completedIndentedLines.replace(
      squareBracesRegex,
      (_, indentation, line, elementsStr) => {
        const maxPropertiesPerLine =
          config.arrayFormatting.maxElementsPerLine;
        // childrenStr = this._splitByTopLevelCommas(childrenStr);
        const elements = elementsStr
        .split(/\s*,\s*/)
        .filter(Boolean)
        .map((prop) => prop.trim());

        console.log(_, indentation, line, elements);
        if (elements.length > maxPropertiesPerLine) {
          // Multi-line formatting
          const formattedChildren = elements
            .map((prop, index) => {
              const comma =
                index < elements.length - 1
                ? ","
                : "";
              return indentation + indentString + prop + comma + "\n";
            })
            .join("");

          return `${indentation}${line}[\n${formattedChildren}${indentation}]`;
        } else {
           // Format inline
           const formattedChildren = elements.map((child, index) => {
  
            const comma =
            index < elements.length - 1
            ? ","
            : ""
            return child + comma;
     
           })
           .join("");
           return `${indentation}${line}[${formattedChildren}]`;
        }
      }
    );

    // // Format round braces
    // completedIndentedLines = completedIndentedLines.replace(
    //   roundBracesRegex,
    //   (_, indentation, line, childrenStr) => {
    //     const maxPropertiesPerLine =
    //       config.objectFormatting.maxPropertiesPerLine;
    //     const children = childrenStr
    //     .split(',')
    //     .map((prop) => prop.trim())
    //     .filter((prop) => prop !== "");
    //     console.log(children, childrenStr);

    //     if (children.length > maxPropertiesPerLine) {
    //       // Multi-line formatting
    //       const formattedChildren = children
    //         .map((prop, index) => {
    //           const comma =
    //             index < children.length - 1
    //             ? ","
    //             : "";
    //           return indentation + indentString + prop + comma + "\n";
    //         })
    //         .join("");

    //       return `${indentation}${line}(\n${formattedChildren}${indentation})`;
    //     } else {
    //         // Format inline
    //         const formattedChildren = children.map((child, index) => {
  
    //         const comma =
    //         index < children.length - 1
    //         ? ","
    //         : ""
    //         return child + comma;
      
    //         })
    //         .join("");
    //         return `${indentation}${line}(${formattedChildren})`;
    //     }
    //   }
    // );

    // Format round braces
    completedIndentedLines = completedIndentedLines.replace(
      roundBracesRegex,
      (match, p1, p2, p3) => {
        // Collect the captured parameters
        const params = [p1, p2, p3]
          .filter((item) => item && item.trim() !== "")
          .map((item) => item.trim());

        // Reconstruct the parameters with trimmed values
        return `(${params.join(", ")})`;
      }
    );

    // // Format curly braces
    // completedIndentedLines = completedIndentedLines.replace(
    //   curlyBracesRegex,
    //   (match, p1, p2, p3) => {
    //     const items = [p1, p2, p3]
    //       .filter((item) => item && item.trim() !== "")
    //       .map((item) => item.trim());
    //     return `{${items.join(", ")}}`;
    //   }
    // );

    completedIndentedLines = completedIndentedLines.replace(interpolationRegex, (match, content) => {
      return '$' + `{${content.trim()}}`;
    })

    completedIndentedLines = completedIndentedLines.replace(
      typeDefinitionRegex,
      (match, indentation, declaration, typeDefinition, restOfLine) => {
        // Remove outer braces and trim
        let typeContent = typeDefinition.slice(1, -1).trim();

        // Split properties by semicolons
        const properties = typeContent
          .split(";")
          .map((prop) => prop.trim())
          .filter((prop) => prop !== "");

        const maxPropertiesPerLine =
          config.objectFormatting.maxPropertiesPerLine;
        const trailingSemicolon = config.typeFormatting.trailingSemicolon;

        let formattedTypeDefinition;

        if (properties.length > maxPropertiesPerLine) {
          // Multi-line formatting
          const formattedProperties = properties
            .map((prop, index) => {
              const semicolonStr = trailingSemicolon
                ? ";"
                : index < properties.length - 1
                ? ";"
                : "";
              return indentation + indentString + prop + semicolonStr + "\n";
            })
            .join("");

          formattedTypeDefinition = `{\n${formattedProperties}${indentation}}`;
        } else {
          // Inline formatting
          const semicolonStr = trailingSemicolon ? ";" : "";
          const formattedProperties = properties.join("; ");
          formattedTypeDefinition = `{ ${formattedProperties}${semicolonStr} }`;
        }

        // Reconstruct the line
        return `${indentation}${declaration}: ${formattedTypeDefinition}${restOfLine}`;
      }
    );

    completedIndentedLines = completedIndentedLines.replace(
      objectAssignRegex,
      (match, indentation, assignment, objectContent) => {
        objectContent = objectContent.trim();

        const children = objectContent
          .split(",")
          .map((child) => child.trim())
          .filter((child) => child !== "");

        if (children.length > config.objectFormatting.maxPropertiesPerLine) {
          // Format with new lines and indentation
          const formattedChildren = children
            .map((child, index) => {
              const comma = config.objectFormatting.trailingComma
                ? ","
                : index < children.length - 1
                ? ","
                : "";
              return indentation + indentString + child + comma + "\n";
            })
            .join("");

          return `${indentation}${assignment}{\n${formattedChildren}${indentation}}`;
        } else {
          // Format inline
          const formattedChildren = children.join(", ");
          const trailingCommaStr = config.objectFormatting.trailingComma
            ? ","
            : "";
          return `${indentation}${assignment}{ ${formattedChildren}${trailingCommaStr} }`;
        }
      }
    );

    // Format imports
    completedIndentedLines = completedIndentedLines.replace(
      importRegex,
      (_, imports, fromPath) => {
        const formattedImports = imports
          .split(",")
          .map((imp) => imp.trim())
          .join(", ");
        return config.importFormat.spacesAroundImports
          ? `import { ${formattedImports} } from '${fromPath}';`
          : `import {${formattedImports}} from '${fromPath}';`;
      }
    );

    return completedIndentedLines;
  }

  /**
   * Function that replaces double quotes with single quotes in the given content.
   * @param content - The content string where double quotes should be replaced.
   * @returns The formatted string with single quotes.
   */
  public replaceDoubleQuotes(content: string): string {
    return content.replace(/"/g, "'");
  }

  private _splitByTopLevelCommas(input: string): string[] {
    const result: string[] = [];
    let currentPart: string = '';
    let depth = 0; // Tracks the depth of nested structures (e.g., parentheses)

    for (let i = 0; i < input.length; i++) {
        const char = input[i];

        if (char === '(' || char === '[' || char === '{') {
            // Increase depth when entering a nested structure
            depth++;
        } else if (char === ')' || char === ']' || char === '}') {
            // Decrease depth when exiting a nested structure
            depth--;
        }

        if (char === ',' && depth === 0) {
            // Only split on commas that are at the top level (not nested)
            result.push(currentPart.trim());
            currentPart = '';
        } else {
            // Append the character to the current part
            currentPart += char;
        }
    }

    // Push the final part (after the last comma)
    if (currentPart.trim() !== '') {
        result.push(currentPart.trim());
    }

    return result;
}
}
