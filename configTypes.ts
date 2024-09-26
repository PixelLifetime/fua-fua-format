export type IndentationConfig = {
    type: IndentationType;
    size: number;
  }
  export type IndentationType = 'spaces' | 'tabs';
  
  export type ImportFormatConfig = {
    spacesAroundImports: boolean;
  }
  
  export type ObjectFormattingConfig = {
    maxPropertiesPerLine: number;
    trailingComma: boolean;
  }

  export type TypeFormatting = {
    maxPropertiesPerLine: number;
    trailingSemicolon: boolean;
  }

  export type ArrayFormatting = {
    maxElementsPerLine: number;
    trailingSemicolon: boolean;
  }
  
  export type FormatterConfig = {
    indentation: IndentationConfig;
    importFormat: ImportFormatConfig;
    maxLineLength: number;
    objectFormatting: ObjectFormattingConfig;
    typeFormatting: TypeFormatting;
    arrayFormatting: ArrayFormatting;
  }
  