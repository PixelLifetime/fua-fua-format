import { FormatterConfig } from "../../configTypes";

class Node {
  type: "root" | "curly" | "square" | "round" | "unknown";
  start: number;
  end: number;
  parent: Node | null;
  children: Node[];
  depth: number;
  formattedContent: string;

  constructor(
    type: "root" | "curly" | "square" | "round" | "unknown",
    start: number,
    parent: Node | null
  ) {
    this.type = type;
    this.start = start;
    this.end = -1; // To be set when the node is closed
    this.parent = parent;
    this.children = [];
    this.depth = 0;
    this.formattedContent = "";
  }
}

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
   * Function that formats the given content by parsing it into a tree of nodes and
   * applying formatting rules starting from the deepest nodes.
   *
   * @param content - The content string (usually code) that needs to be formatted.
   * @param config - The configuration object that dictates the formatting rules (e.g., indentation size, type, object formatting).
   * @returns The formatted content string with proper indentation, new lines, and spacing.
   */
  public fixIndentation(content: string, config: FormatterConfig): string {

    const rootNode = this.parseCodeToTree(content);

    this.assignDepthLevels(rootNode, 0);

    const formattedContent = this.formatNode(rootNode, content, config);

    // Return the formatted content
    return formattedContent;
  }

  private parseCodeToTree(content: string): Node {
    const rootNode = new Node("root", 0, null);
    let currentNode = rootNode;
    const stack: Node[] = [];
    let i = 0;

    let inSingleQuoteString = false;
    let inDoubleQuoteString = false;
    let inTemplateString = false;
    let inSingleLineComment = false;
    let inMultiLineComment = false;

    while (i < content.length) {
      const char = content[i];
      const prevChar = i > 0 ? content[i - 1] : "";

      // Handle comments and strings
      if (inSingleLineComment) {
        if (char === "\n") {
          inSingleLineComment = false;
        }
      } else if (inMultiLineComment) {
        if (char === "*" && content[i + 1] === "/") {
          inMultiLineComment = false;
          i++; // Skip '/'
        }
      } else if (inSingleQuoteString) {
        if (char === "'" && prevChar !== "\\") {
          inSingleQuoteString = false;
        }
      } else if (inDoubleQuoteString) {
        if (char === '"' && prevChar !== "\\") {
          inDoubleQuoteString = false;
        }
      } else if (inTemplateString) {
        if (char === "`" && prevChar !== "\\") {
          inTemplateString = false;
        }
      } else {

        if (char === "/" && content[i + 1] === "/") {
          inSingleLineComment = true;
          i++; // Skip '/'
        } else if (char === "/" && content[i + 1] === "*") {
          inMultiLineComment = true;
          i++; // Skip '*'
        } else if (char === "'") {
          inSingleQuoteString = true;
        } else if (char === '"') {
          inDoubleQuoteString = true;
        } else if (char === "`") {
          inTemplateString = true;
        } else if (char === "{" || char === "(" || char === "[") {
  
          const type =
            char === "{" ? "curly" : char === "(" ? "round" : "square";
          const newNode = new Node(type, i, currentNode);
          currentNode.children.push(newNode);
          stack.push(currentNode);
          currentNode = newNode;
        } else if (char === "}" || char === ")" || char === "]") {

          currentNode.end = i;
          currentNode = stack.pop() || rootNode;
        }
      }
      i++;
    }

    return rootNode;
  }

  private assignDepthLevels(node: Node, depth: number) {
    node.depth = depth;
    for (const child of node.children) {
      this.assignDepthLevels(child, depth + 1);
    }
  }

  private formatNode(
    node: Node,
    content: string,
    config: FormatterConfig
  ): string {

    if (node.children.length === 0) {
      node.formattedContent = content.substring(node.start, node.end + 1);
      return node.formattedContent;
    }

    let result = "";
    let lastIndex = node.start + 1;

    for (const child of node.children) {
      const beforeChild = content.substring(lastIndex, child.start);
      result += beforeChild;

      const formattedChild = this.formatNode(child, content, config);
      result += formattedChild;

      lastIndex = child.end + 1;
    }


    const afterLastChild = content.substring(lastIndex, node.end);
    result += afterLastChild;

    let innerContent = result.trim();

    if (
      node.type === "curly" ||
      node.type === "square" ||
      node.type === "round"
    ) {
      // Split into lines
      let lines = innerContent.split("\n");

      // Indent lines based on node depth
      const indentSize = config.indentation.size;
      const indentType = config.indentation.type;
      const indentString =
        indentType === "tabs" ? "\t" : " ".repeat(indentSize);
      const indent = indentString.repeat(node.depth);

      for (let i = 0; i < lines.length; i++) {
        lines[i] = indent + lines[i].trim();
      }

      // Reconstruct the content
      innerContent =
        "\n" + lines.join("\n") + "\n" + indentString.repeat(node.depth - 1);
    }

    // Add the opening and closing braces
    const openBrace = content[node.start];
    const closeBrace = content[node.end];

    node.formattedContent = openBrace + innerContent + closeBrace;
    return node.formattedContent;
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
    let currentPart: string = "";
    let depth = 0; // Tracks the depth of nested structures (e.g., parentheses)

    for (let i = 0; i < input.length; i++) {
      const char = input[i];

      if (char === "(" || char === "[" || char === "{") {
        // Increase depth when entering a nested structure
        depth++;
      } else if (char === ")" || char === "]" || char === "}") {
        // Decrease depth when exiting a nested structure
        depth--;
      }

      if (char === "," && depth === 0) {
        // Only split on commas that are at the top level (not nested)
        result.push(currentPart.trim());
        currentPart = "";
      } else {
        // Append the character to the current part
        currentPart += char;
      }
    }

    // Push the final part (after the last comma)
    if (currentPart.trim() !== "") {
      result.push(currentPart.trim());
    }

    return result;
  }
}