// 10. Format imports and class structures
function formatImportsAndClass(content, indentSize = 4) {
  const importRegex = /import\s*{?\s*([^}]*?)\s*}?\s*from\s*['"]([^'"]+)['"]/g;
  content = content.replace(importRegex, (match, imports, fromPath) => {
    const formattedImports = imports
      .split(",")
      .map((imp) => imp.trim())
      .join(", ");
    return `import { ${formattedImports} } from '${fromPath}'`;
  });

  content = content.replace(/^\s*import\s+/gm, "import ");

  const classRegex =
    /(@Injectable\([\s\S]*?\))\s*(export\s+class\s+\w+\s*{[\s\S]*?})/g;
  content = content.replace(classRegex, (match, decorator, classBlock) => {
    return `${decorator}\n${classBlock
      .replace(/\n\s*\n/g, "\n")
      .replace(/\s*{\s*/g, " {\n")
      .replace(/;(\s*\n)/g, ";\n")
      .replace(/\n\s+/g, (match) => match.trimStart())
      .replace(/(\})\s*$/, "$1\n")}`;
  });
  const injectableRegex = /(@Injectable\([\s\S]*?\)[\s\S]*?;?)/g;
  
  content = content.replace(injectableRegex, (match, decorator) => {
    const indentedLines: string[] = [];
    let currentIndentLevel = 0;

    // Split the decorator into lines
    const lines = decorator.split('\n');

    lines.forEach(line => {
      line = line.replace(';', '');
      // Adjust the indentation based on opening and closing braces
      if (line.includes('}')) {
        currentIndentLevel -= 1;
      }

      // Add the current line with the appropriate indentation
      indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);
      console.log(line, currentIndentLevel, indentSize);

      if (line.endsWith('{')) {
        currentIndentLevel += 1;
      }

    });

    // Join the lines back together
    return indentedLines.join('\n');
  });
  return content;
}