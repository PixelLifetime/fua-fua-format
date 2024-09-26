// 9. Fix if statements spacing
function formatIfStatements(content) {
  const ifRegex = /if\s*\(\s*([^\)]+?)\s*\)\s*{/g;

  return content.replace(ifRegex, (match, condition) => {
    return `if (${condition.trim()}) {`;
  });
}
module.exports = formatIfStatements
