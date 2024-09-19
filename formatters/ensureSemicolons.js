// 2. Ensure all lines end with a semicolon, but skip object literals and commas
function ensureSemicolons(content) {
    return content
      .split("\n")
      .map((line) => {
        line = line.trim();
        if (
          line.endsWith(",") ||
          line.endsWith("{") ||
          line.endsWith("}") ||
          line.endsWith("(") ||
          line.endsWith("=>") ||
          line.includes("return {") ||
          line.includes("...")
        ) {
          return line;
        }
  
        if (
          line &&
          !line.endsWith(";") &&
          !line.startsWith("/") &&
          !line.startsWith("*")
        ) {
          return line + ";";
        }
  
        return line;
      })
      .join("\n");
  }
module.exports = ensureSemicolons;