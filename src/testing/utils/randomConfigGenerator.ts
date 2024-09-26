import {
  ArrayFormatting,
  FormatterConfig,
  ImportFormatConfig,
  IndentationConfig,
  IndentationType,
  ObjectFormattingConfig,
  TypeFormatting,
} from "../../../configTypes";

export class RandomConfigGenerator {
  
  /**
   * Generates a random integer between min and max (inclusive).
   * @param min - Minimum value.
   * @param max - Maximum value.
   * @returns A random integer.
   */
  private _getRandomInt(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }

  /**
   * Randomly selects an element from an array.
   * @param array - Array of elements to choose from.
   * @returns A randomly selected element.
   */
  private _getRandomElement<T>(array: T[]): T {
    const index = Math.floor(Math.random() * array.length);
    return array[index];
  }

  /**
   * Generates a random FormatterConfig object.
   * @returns A FormatterConfig with randomized properties.
   */
  public generateRandomConfig(): FormatterConfig {
    // Define possible values for each configurable property
    const indentationTypes: IndentationType[] = ["spaces", "tabs"];
    const objectMaxPropertiesRange = { min: 1, max: 5 };
    const maxLineLengthRange = { min: 60, max: 120 };

    // Generate random indentation settings
    const indentation: IndentationConfig = {
      type: this._getRandomElement(indentationTypes),
      size: indentationTypes.includes("spaces") ? this._getRandomInt(2, 8) : 1, // Typically, tabs size is 1
    };

    // Generate random import formatting settings
    const importFormat: ImportFormatConfig = {
      spacesAroundImports: Math.random() < 0.5,
    };

    // Generate random object formatting settings
    const objectFormatting: ObjectFormattingConfig = {
      maxPropertiesPerLine: this._getRandomInt(
        objectMaxPropertiesRange.min,
        objectMaxPropertiesRange.max
      ),
      trailingComma: Math.random() < 0.5,
    };

    // Generate random type formatting settings
    const typeFormatting: TypeFormatting = {
      maxPropertiesPerLine: this._getRandomInt(
        objectMaxPropertiesRange.min,
        objectMaxPropertiesRange.max
      ),
      trailingSemicolon: Math.random() < 0.5,
    };

    // Generate random array formatting settings
    const arrayFormatting: ArrayFormatting = {
      maxElementsPerLine: this._getRandomInt(
        objectMaxPropertiesRange.min,
        objectMaxPropertiesRange.max
      ),
      trailingSemicolon: Math.random() < 0.5,
    };

    // Generate random maximum line length
    const maxLineLength: number = this._getRandomInt(
      maxLineLengthRange.min,
      maxLineLengthRange.max
    );

    // Assemble the FormatterConfig object
    const config: FormatterConfig = {
      indentation,
      importFormat,
      maxLineLength,
      objectFormatting,
      typeFormatting,
      arrayFormatting
    };

    return config;
  }
}
