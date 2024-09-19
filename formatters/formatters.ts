export function fixTypeSignatureSpacing(content: string): string {
  return content.replace(/\s*:\s*/g, ": ");
}

export function fixIndentation(content: string, config: any): string {
  const lines = content.split("\n");
  const indentedLines: string[] = [];
  let currentIndentLevel = 0;
  const indentString = " ".repeat(config.indentation.size);
  const functionDeclarationRegex = /(\w+\([^)]*\)\s*)(\s*:\s*[^{]*)?{/;
  const controlStructuresRegex = /\w+\s+\(([^,)]+)\)\s+\{/;
  const squareBracesRegex =
    /\[\s*([^,\[\]]+)?(?:\s*,\s*([^,\[\]]+))?(?:\s*,\s*([^,\[\]]+))?\s*\]/g;
  const roundBracesRegex =
    /\(\s*([^,()]+)?(?:\s*,\s*([^,()]+))?(?:\s*,\s*([^,()]+))?\s*\)/g;
  const curlyBracesRegex =
    /\{\s*([^,;{}]+)?(?:\s*,\s*([^,;{}]+))?(?:\s*,\s*([^,;{}]+))?\s*\}/g;

  const objectAssignRegex = /([ \t]*)(\b(?:const|let|var)\s+\w+(?:\s*:\s*(?:\{[\s\S]*?\} | \w+))?\s*=\s*)\{\s*((?:[^{}]*?(?:,)\s*){1,})\}/g;
  const typeDefinitionRegex = /^([ \t]*)(\b(?:const|let|var)\s+\w+)\s*:\s*(\{[\s\S]*?\})(.*$)/gm;

  const countChildrenInObject = /\{\s*((?:[^{}]*?(?:,)\s*){3,})\}/;

  const interpolationRegex = /\$\{(\s*.*\s*)\}/g;
  const importRegex =
    /import\s*{?\s*([^}]*?)\s*}?\s*from\s*['"]([^'"]+)['"](;)?/g;

    for (let line of lines) {
        line = line.trim(); // Trim the line
        let resultLine = indentString.repeat(currentIndentLevel); // Start with current indentation
        let i = 0;
      
        // Skip empty lines if desired
        if (line === '') {
          indentedLines.push('');
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
      
          if (char === '{' || char === '(' || char === '[') {
            // Add the character to resultLine
            resultLine += char;
            i++;
      
            // Increase indent level
            currentIndentLevel++;
      
            // Check if there's already a newline after the opening brace
            if (i < line.length && (line[i] === '\n' || line[i] === '\r')) {
              // There's already a newline, so we don't need to add one
              while (i < line.length && (line[i] === '\n' || line[i] === '\r')) {
                // Add existing newline characters to resultLine
                resultLine += line[i];
                i++;
              }
              // Start a new line with increased indentation
              resultLine += indentString.repeat(currentIndentLevel);
            } else {
              // No newline present, so we add one
              // Push the current line to indentedLines
              if (resultLine.trim() !== '') {
                indentedLines.push(resultLine);
              }
              // Start a new line with increased indentation
              resultLine = indentString.repeat(currentIndentLevel);
            }
          } else if (char === '}' || char === ')' || char === ']') {
            // Push the current line to indentedLines if not empty
            if (resultLine.trim() !== '') {
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
        if (resultLine.trim() !== '') {
          indentedLines.push(resultLine);
        }
      }
      

  let completedIndentedLines = indentedLines.join("\n");

  // Format square braces
  completedIndentedLines = completedIndentedLines.replace(
    squareBracesRegex,
    (match, p1, p2, p3) => {
      // Collect the captured items
      const items = [p1, p2, p3]
        .filter((item) => item && item.trim() !== "")
        .map((item) => item.trim());

      // Reconstruct the array with trimmed items
      return `[${items.join(", ")}]`;
    }
  );

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

  // Format curly braces
  completedIndentedLines = completedIndentedLines.replace(
    curlyBracesRegex,
    (match, p1, p2, p3) => {
      const items = [p1, p2, p3]
        .filter((item) => item && item.trim() !== "")
        .map((item) => item.trim());
      return `{${items.join(", ")}}`;
    }
  );

  completedIndentedLines = completedIndentedLines.replace(
    typeDefinitionRegex,
    (match, indentation, declaration, typeDefinition, restOfLine) => {
      // Remove outer braces and trim
      let typeContent = typeDefinition.slice(1, -1).trim();
  
      // Split properties by semicolons
      const properties = typeContent
        .split(';')
        .map(prop => prop.trim())
        .filter(prop => prop !== '');
  
      const minPropertiesPerLine = config.objectFormatting.maxPropertiesPerLine || 3;
      const semicolon = config.objectFormatting.trailingSemicolon || false;
      const indentString = config.indentation.type === 'tabs' ? '\t' : ' '.repeat(config.indentation.size || 4);
  
      let formattedTypeDefinition;
  
      if (properties.length > minPropertiesPerLine) {
        // Multi-line formatting
        const formattedProperties = properties.map((prop, index) => {
          const semicolonStr = semicolon ? ';' : index < properties.length - 1 ? ',' : '';
          return indentation + indentString + prop + semicolonStr + '\n';
        }).join('');
  
        formattedTypeDefinition = `{\n${formattedProperties}${indentation}}`;
      } else {
        // Inline formatting
        const semicolonStr = semicolon ? ';' : '';
        const formattedProperties = properties.join('; ');
        formattedTypeDefinition = `{ ${formattedProperties}${semicolonStr} }`;
      }
  
      // Reconstruct the line
      return `${indentation}${declaration}: ${formattedTypeDefinition}${restOfLine}`;
    }
  );  

  completedIndentedLines = completedIndentedLines.replace(
    objectAssignRegex,
    (match, indentation:string, assignment, objectContent) => {
      objectContent = objectContent.trim();
      console.log(indentation.length);
      
      const children = objectContent
        .split(',')
        .map(child => child.trim())
        .filter(child => child !== '');
  
      if (children.length > config.objectFormatting.maxPropertiesPerLine) {
        // Format with new lines and indentation
        const formattedChildren = children.map((child, index) => {
          const comma = config.objectFormatting.trailingComma ? ',' : index < children.length - 1 ? ',' : '';
          return indentation + indentString + child + comma + '\n';
        }).join('');
  
        return `${indentation}${assignment}{\n${formattedChildren}${indentation}}`;
      } else {
        // Format inline
        const formattedChildren = children.join(', ');
        const trailingCommaStr = config.objectFormatting.trailingComma ? ',' : '';
        return `${assignment}{ ${formattedChildren}${trailingCommaStr} }`;
      }
    }
  );

  //   // Format interpolation
  //   completedIndentedLines = completedIndentedLines.replace(
  //     interpolationRegex,
  //     (_, p1) => {
  //       return "${" + p1.trim() + "}";
  //     }
  //   );

  // Format imports
  completedIndentedLines = completedIndentedLines.replace(
    importRegex,
    (_, imports, fromPath) => {
      const formattedImports = imports
        .split(",")
        .map((imp) => imp.trim())
        .join(", ");
      return `import { ${formattedImports} } from '${fromPath}';`;
    }
  );

  return completedIndentedLines;
}

// function getIndentLevel(line, indentSize) {
//   const match = line.match(/^([ \t]*)/);
//   const indent = match ? match[1] : '';
//   const indentLength = indent.replace(/\t/g, ' '.repeat(indentSize)).length;
//   return Math.floor(indentLength / indentSize);
// }

// function objectAssignReplacement(match, assignment, objectContent) {
//   objectContent = objectContent.trim();

//   const children = objectContent
//     .split(',')
//     .map(child => child.trim())
//     .filter(child => child !== '');

//   const indentation = indentString.repeat(currentIndentLevel);

//   if (children.length > config.objectFormatting.maxPropertiesPerLine) {
//     // Format with new lines and indentation
//     const formattedChildren = children.map((child, index) => {
//       const comma = config.objectFormatting.trailingComma ? ',' : index < children.length - 1 ? ',' : '';
//       return indentation + indentString + child + comma + '\n';
//     }).join('');

//     return `${indentation}${assignment}{\n${formattedChildren}${indentation}};`;
//   } else {
//     // Format inline
//     const formattedChildren = children.join(', ');
//     const trailingCommaStr = config.objectFormatting.trailingComma ? ',' : '';
//     return `${indentation}${assignment}{ ${formattedChildren}${trailingCommaStr} };`;
//   }
// }

// function typeDefinitionReplacement(match, declaration, typeDefinition, restOfLine) {
//   let typeContent = typeDefinition.slice(1, -1).trim();

//   const properties = typeContent
//     .split(';')
//     .map(prop => prop.trim())
//     .filter(prop => prop !== '');

//   const indentation = indentString.repeat(currentIndentLevel);

//   if (properties.length > config.objectFormatting.maxPropertiesPerLine) {
//     // Multi-line formatting
//     const formattedProperties = properties.map((prop, index) => {
//       const semicolonStr = config.objectFormatting.trailingSemicolon ? ';' : index < properties.length - 1 ? ',' : '';
//       return indentation + indentString + prop + semicolonStr + '\n';
//     }).join('');

//     const semicolonStr = config.objectFormatting.semicolon ? ';' : '';
//     const formattedTypeDefinition = `{\n${formattedProperties}${indentation}}`;

//     return `${indentation}${declaration}: ${formattedTypeDefinition}${restOfLine}`;
//   } else {
//     // Inline formatting
//     const semicolonStr = config.objectFormatting.trailingSemicolon ? ';' : '';
//     const formattedProperties = properties.join('; ');
//     const formattedTypeDefinition = `{ ${formattedProperties}${semicolonStr} }`;

//     return `${indentation}${declaration}: ${formattedTypeDefinition}${restOfLine}`;
//   }
// }

export default function generateFormattedCode(code, options) {
  const indentSize = options.indentation.size;
  const indentType = options.indentation.type;
  const indentString = indentType === 'tabs' ? '\t' : ' '.repeat(indentSize);

  code = code.replace(/\n/g, ' ').replace(/\s+/g, ' ');

  // Regular expression to match the variable declaration
  const variableDeclarationRegex = /(const|let|var)\s+(\w+)(\s*:\s*\{[^}]+\})?\s*=\s*(\{[^}]+\});?/;

  const match = code.match(variableDeclarationRegex);

  if (!match) {
    return code;
  }

  const [fullMatch, varKeyword, varName, typeAnnotation, initializer] = match;

  let formattedTypeAnnotation = '';
  if (typeAnnotation) {
    const typeContent = typeAnnotation.slice(typeAnnotation.indexOf('{') + 1, typeAnnotation.lastIndexOf('}'));
    const typeProperties = typeContent.split(/\s*[,;]\s*/).filter(Boolean);

    const useMultiline = typeProperties.length >= options.maxPropertiesPerLine;

    if (useMultiline) {
      formattedTypeAnnotation = `: {\n`;
      typeProperties.forEach((prop, index) => {
        prop = prop.trim();
        const semicolon = options.semicolon || index < typeProperties.length - 1 ? ';' : '';
        formattedTypeAnnotation += `${indentString}${prop}${semicolon}\n`;
      });
      formattedTypeAnnotation += `}`;
    } else {
      const semicolon = options.semicolon ? ';' : '';
      formattedTypeAnnotation = `: { ${typeProperties.join('; ')}${semicolon} }`;
    }
  }

  const initContent = initializer.slice(initializer.indexOf('{') + 1, initializer.lastIndexOf('}'));
  const initProperties = initContent.split(/\s*,\s*/).filter(Boolean);

  const useMultilineInit = initProperties.length >= options.maxPropertiesPerLine;

  let formattedInitializer = '';
  if (useMultilineInit) {
    formattedInitializer = `= {\n`;
    initProperties.forEach((prop, index) => {
      prop = prop.trim();
      const comma = options.trailingComma || index < initProperties.length - 1 ? ',' : '';
      formattedInitializer += `${indentString}${prop}${comma}\n`;
    });
    formattedInitializer += `}`;
  } else {
    const comma = options.trailingComma ? ',' : '';
    formattedInitializer = `= { ${initProperties.join(', ')}${comma} }`;
  }

  const formattedCode = `${varKeyword} ${varName}${formattedTypeAnnotation} ${formattedInitializer}`;

  return formattedCode;
}
