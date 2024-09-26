export function formatFunctionalOperators(content) {
  const lines = content.split("\n");
  const indentedLines: string[] = [];
  let currentIndentLevel = 0;
  let functionCallLevel = 0; // Track the level of nested function calls
  let insideBlockComment = false; // To track if we are inside a multi-line block comment

  // List of common functional operators
  const functionalOperators = [
    "map",
    "forEach",
    "pipe",
    "tap",
    "filter",
    "reduce",
    "switchMap",
    "subscribe",
    "findIndex",
  ];

  // Helper function to check if the line contains one of the functional operators
  function isFunctionalOperator(line) {
    return functionalOperators.some((op) => line.includes(`${op}(`));
  }

  lines.forEach((line) => {
    let trimmedLine = line.trim();

    // Ignore multi-line block comments
    if (trimmedLine.startsWith("/*")) {
      insideBlockComment = true;
    }
    if (insideBlockComment) {
      indentedLines.push(line); // Add the block comment line as-is
      if (trimmedLine.endsWith("*/")) {
        insideBlockComment = false;
      }
      return;
    }

    // Ignore single-line comments
    if (trimmedLine.startsWith("//")) {
      indentedLines.push(line); // Add the single-line comment as-is
      return;
    }

    // If the line contains a functional operator (map, pipe, forEach, etc.)
    if (isFunctionalOperator(trimmedLine)) {
      const operatorMatch = trimmedLine.match(/(\w+)\s*\(/);
      if (operatorMatch) {
        const functionName = operatorMatch[1];

        // Find the index of the last opening parenthesis '(' to properly handle the argument list
        const operatorIndex = trimmedLine.indexOf(functionName);
        const beforeParen = trimmedLine.slice(
          0,
          operatorIndex + functionName.length + 1
        ); // Everything before and including '('
        const afterParen = trimmedLine
          .slice(operatorIndex + functionName.length + 1)
          .trim(); // Everything after '('

        // Add the part before the operator and the operator itself
        indentedLines.push(beforeParen);
        currentIndentLevel++; // Increase indent for inner content
        functionCallLevel++; // Increase the function call level

        // Add the content after the opening parenthesis, if it exists
        if (afterParen.length > 0) {
          indentedLines.push(afterParen);
        }
        return;
      }
    }

    // Handle the closing of a functional operator block, especially if it ends with `});`
    if (
      functionCallLevel > 0 &&
      (trimmedLine.endsWith("});") || trimmedLine.endsWith("}),"))
    ) {
      const closingIndex = trimmedLine.lastIndexOf("}");
      const beforeClose = trimmedLine.slice(0, closingIndex + 1); // Everything before the closing brace
      const afterClose = trimmedLine.slice(closingIndex + 1).trim(); // Everything after the closing brace (i.e., `);`)

      // Add the content before the closing parenthesis and move `);` to a new line
      indentedLines.push(beforeClose);
      currentIndentLevel--; // Decrease indent for `);`
      indentedLines.push(afterClose);
      functionCallLevel--; // Decrease the function call level
      return;
    }

    // For any other lines, add them as-is with the current indentation
    indentedLines.push(trimmedLine);
  });

  return indentedLines.join("\n");
}
