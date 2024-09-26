import * as fs from "fs";
import * as path from "path";
import * as chalk from "chalk";

import { FormatterConfig } from "../../configTypes";
import { Formatters } from "../formatting/formatters";
import { FormattedCodeGenerators } from "./utils/formattedCodeGenerators";

export class Tests {
    
  /**
   * Class that contains functions for formatting code
   */
  private _formatters: Formatters = new Formatters();

  /**
   * Class that contains functions for generating properly formatted code
   */
  private _formattedCodeGenerators: FormattedCodeGenerators =
    new FormattedCodeGenerators();

  /**
   * Tests the object type and assignment formatting using the custom formatter.
   * Reads the content of a specific TypeScript file, formats it using both the custom
   * formatter and a pre-generated expected output, then compares them.
   * Logs the result and returns whether the formatting test passed or failed.
   *
   * @param randomConfig - The random configuration to be used for formatting.
   * @returns `true` if the generated and custom formatted code match, otherwise `false`.
   */
  public objectTypeAndAssignmentFormatting(
    randomConfig: FormatterConfig
  ): boolean {
    try {
      const absolutePath = path.resolve(
        "testing/testFiles/objectTypeAndAssignment.ts"
      );
      const tsContent = fs.readFileSync(absolutePath, "utf-8");
      let formattedTs = "";
      formattedTs = this._formatters.fixIndentation(tsContent, randomConfig);
      const generatedFormattedCode =
        this._formattedCodeGenerators.formattedObjectTypeAndAssignment(
          tsContent,
          randomConfig
        );

      const formattingSucceed = generatedFormattedCode === formattedTs;

      fs.writeFileSync(absolutePath, formattedTs);
      console.log(chalk.blue(`Custom Formatted TS file: ${absolutePath}`));

      if (formattingSucceed) {
        console.log(
          chalk.green("Object type and assignment formatting works!")
        );
        return true;
      } else {
        console.error(
          chalk.red(
            "Ohh, something went wrong.\nObject type and assignment formatting test failed!"
          )
        );

        console.log(
          chalk.magenta("Generated formatted code: \n"),
          JSON.stringify(generatedFormattedCode)
        );
        console.log(
          chalk.cyan("Formatted code: \n"),
          JSON.stringify(formattedTs)
        );
        return false;
      }
    } catch (error) {
      console.error(
        chalk.red(`Error processing file testing/testFiles/objectTypeAndAssignment.ts:`),
        error
      );
      return false;
    }
  }

    /**
   * Tests the array formatting using the custom formatter.
   * Reads the content of a specific TypeScript file, formats it using both the custom
   * formatter and a pre-generated expected output, then compares them.
   * Logs the result and returns whether the formatting test passed or failed.
   *
   * @param randomConfig - The random configuration to be used for formatting.
   * @returns `true` if the generated and custom formatted code match, otherwise `false`.
   */
    public arrayFormatting(
      randomConfig: FormatterConfig
    ): boolean {
      try {
        const absolutePath = path.resolve(
          "testing/testFiles/array.ts"
        );
        const tsContent = fs.readFileSync(absolutePath, "utf-8");
        let formattedTs = this._formatters.fixIndentation(tsContent, randomConfig);
        const generatedFormattedCode =
          this._formattedCodeGenerators.formattedArray(
            tsContent,
            randomConfig
          );
  
        const formattingSucceed = generatedFormattedCode === formattedTs;
  
        fs.writeFileSync(absolutePath, formattedTs);
        console.log(chalk.blue(`Custom Formatted TS file: ${absolutePath}`));
  
        if (formattingSucceed) {
          console.log(
            chalk.green("Array formatting works!")
          );
          return true;
        } else {
          console.error(
            chalk.red(
              "Ohh, something went wrong.\nArray formatting test failed!"
            )
          );
  
          console.log(
            chalk.magenta("Generated formatted code: \n"),
            JSON.stringify(generatedFormattedCode)
          );
          console.log(
            chalk.cyan("Formatted code: \n"),
            JSON.stringify(formattedTs)
          );
          return false;
        }
      } catch (error) {
        console.error(
          chalk.red(`Error processing file testing/testFiles/array.ts:`),
          error
        );
        return false;
      }
    }

  /**
   * Tests the imports formatting using the custom formatter.
   * Reads the content of a specific TypeScript file, formats it using both the custom
   * formatter and a pre-generated expected output, then compares them.
   * Logs the result and returns whether the formatting test passed or failed.
   * 
   * @param randomConfig - The random configuration to be used for formatting.
   * @returns `true` if the generated and custom formatted code match, otherwise `false`.
   */
  public importFormatting(
    randomConfig: FormatterConfig
  ): boolean {
    try {
      const absolutePath = path.resolve(
        "testing/testFiles/imports.ts"
      );
      const tsContent = fs.readFileSync(absolutePath, "utf-8");
      let formattedTs = "";
      formattedTs = this._formatters.fixIndentation(tsContent, randomConfig);
      const generatedFormattedCode =
        this._formattedCodeGenerators.formattedImports(
          tsContent,
          randomConfig
        );

      const formattingSucceed = generatedFormattedCode === formattedTs;

      fs.writeFileSync(absolutePath, formattedTs);
      console.log(chalk.blue(`Custom Formatted TS file: ${absolutePath}`));

      if (formattingSucceed) {
        console.log(
          chalk.green("Import formatting works!")
        );
        return true;
      } else {
        console.error(
          chalk.red(
            "Ohh, something went wrong.\nImport formatting test failed!"
          )
        );

        console.log(
          chalk.magenta("Generated formatted code: \n"),
          JSON.stringify(generatedFormattedCode)
        );
        console.log(
          chalk.cyan("Formatted code: \n"),
          JSON.stringify(formattedTs)
        );
        return false;
      }
    } catch (error) {
      console.error(
        chalk.red(`Error processing file testing/testFiles/imports.ts:`),
        error
      );
      return false;
    }
  }
}
