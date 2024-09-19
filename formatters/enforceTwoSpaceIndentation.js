// 3. Enforce 2 spaces indentation for tabs
function enforceTwoSpaceIndentation(content) {
  return content.replace(/\t/g, "  ");
}
module.exports = enforceTwoSpaceIndentation
