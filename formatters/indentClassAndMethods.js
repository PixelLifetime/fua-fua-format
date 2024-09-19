function indentClassAndMethods(content, indentSize = 4) {
  const lines = content.split("\n");
  const indentedLines = [];
  let insideFunction = false;
  let insideClass = false; // Track if we're inside a class
  let currentIndentLevel = 0;
  let roundParentCount = 0; // Track open and close parentheses
  let curlyParentCount = 0; // Track open and close parentheses

  // Default control structures that don't require parentheses tracking
  const controlStructures = ["if", "for", "while", "switch", "do"];

  // Regex to detect function declarations (methods inside classes)
  const methodDeclarationRegex =
    /\b(private|public|protected)\s+\w+\s*\(.*\)\s*(?::{1}.*)?\s*{?/;

  // Regex to detect class declarations
  const classDeclarationRegex = /\b(class|interface)\s+\w+/;

  lines.forEach((line) => {
    const trimmedLine = line.trim();

    // Detect the start of a class using the regex
    if (classDeclarationRegex.test(trimmedLine)) {
      insideClass = true;
      currentIndentLevel = 1; // Set the indent level for class declarations
      curlyParentCount = 0; // Reset brace count at the start of a new class
      indentedLines.push(line); // Add indent to the class declaration
      return;
    }

    // Detect the start of a function inside a class using the regex
    if (methodDeclarationRegex.test(trimmedLine)) {
      insideFunction = true;
      currentIndentLevel = 2; // Reset the indent level to 1 for function declarations
      curlyBraceCount = 0; // Reset brace count at the start of a new function
      indentedLines.push(" ".repeat(indentSize) + line); // Add indent to the function declaration
      return;
    }

    // Handle comments (single-line comments and block comments)
    if (
      trimmedLine.startsWith("//") ||
      trimmedLine.startsWith("*") ||
      trimmedLine.startsWith("/*") ||
      trimmedLine.startsWith("*/")
    ) {
      indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);
      return;
    }

    // Inside a function, add indentation for each line
    if (insideFunction) {
      // Check if the line starts with a control structure (if, for, etc.)
      const isControlStructure = controlStructures.some((keyword) =>
        trimmedLine.startsWith(keyword)
      );

      // Track parentheses only for function calls and non-control structures
      if (!isControlStructure) {
        roundParentCount = 0;
        roundParentCount += (trimmedLine.match(/\(/g) || []).length;
        roundParentCount -= (trimmedLine.match(/\)/g) || []).length;
      }

      if (
        trimmedLine.includes("{") &&
        trimmedLine.includes("}") &&
        trimmedLine.includes("else") &&
        !trimmedLine.includes("$")
      ) {
        currentIndentLevel = Math.max(currentIndentLevel - 1, 0); // Ensure indent level doesn't go below 0
        curlyParentCount -= 1;
        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);
        curlyParentCount += 1;
        currentIndentLevel++; // Increase indent after opening curly braces
      }
      // Handle opening braces
      else if (
        trimmedLine.includes("{") &&
        !trimmedLine.includes("$") &&
        !trimmedLine.includes("}")
      ) {
        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);
        curlyParentCount += 1;
        currentIndentLevel++; // Increase indent after opening curly braces
      }
      // Handle closing braces
      else if (
        trimmedLine.includes("}") &&
        !trimmedLine.includes("$") &&
        !trimmedLine.includes("{")
      ) {
        currentIndentLevel = Math.max(currentIndentLevel - 1, 0); // Ensure indent level doesn't go below 0
        curlyParentCount -= 1;
        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);

        if (currentIndentLevel === 1) {
          insideFunction = false; // End of the function
          currentIndentLevel = 1;
        }
      } else if (roundParentCount < 0 && trimmedLine.includes(")")) {
        currentIndentLevel = Math.max(currentIndentLevel - 1, 0); // Ensure indent level doesn't go below 0

        indentedLines.push(
          " ".repeat(Math.max(currentIndentLevel, 0) * indentSize) + line
        ); // Ensure valid indentation
      } else if (roundParentCount > 0 && trimmedLine.includes("(")) {
        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);

        currentIndentLevel++; // Increase indent after opening curly braces
      } else {
        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);
      }

      return;
    }

    if (insideClass) {
      if (
        trimmedLine.includes("}") &&
        !trimmedLine.includes("{") &&
        !trimmedLine.includes("$")
      ) {
        currentIndentLevel = Math.max(currentIndentLevel - 1, 0); // Ensure indent level doesn't go below 0

        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);

        if (currentIndentLevel === 0) {
          insideClass = false; // End of the class
        }
      } else {
        indentedLines.push(" ".repeat(currentIndentLevel * indentSize) + line);
      }

      return;
    }

    // Outside functions, no change
    indentedLines.push(line);
  });

  return indentedLines.join("\n");
}
module.exports = indentClassAndMethods;
