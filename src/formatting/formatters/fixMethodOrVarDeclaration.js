function fixAccessModifierSpacing(content) {
  return content.replace(
    /\b(private|public|protected)\s+([_a-zA-Z])/g,
    "$1 $2"
  );
}

function fixMethodNameSpacing(content) {
  return content.replace(/(\w+)\s+\(\)/g, "$1()");
}

function fixTypeSignatureSpacing(content) {
  return content.replace(/\s*:\s*/g, ': ');
}

function fixDeclaration(content) {
  content = fixAccessModifierSpacing(content);
  content = fixMethodNameSpacing(content);
  content = fixTypeSignatureSpacing(content);
  return content;
}
module.exports = fixDeclaration;
