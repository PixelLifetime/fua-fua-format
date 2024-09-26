import { Command } from "commander";
import * as chalk from "chalk";
import { Tester } from "./testing/tester";
import { Formatter } from "./formatting/formatter";

const program = new Command();

program
  .name("formatter-cli")
  .description("CLI tool for formatting and testing TypeScript and HTML files")
  .version("1.0.0");

// 'format' command
program
  .command("format")
  .description("Format files based on the provided glob pattern")
  .option(
    "-p, --pattern <pattern>",
    "Glob pattern to match files",
    "**/*.{html,ts}"
  )
  .action(async (options) => {
    try {
      const formatter = new Formatter();
      await formatter.formatFiles(options.pattern);
    } catch (error) {
      console.error(chalk.red(`Error during formatting: ${error.message}`));
      process.exit(1);
    }
  });

// 'test' command
program
  .command("test")
  .description("Run tests on files based on the provided glob pattern")
  .action(async () => {
    const tester = new Tester();
    await tester.runTests();
  });

// Parse command-line arguments
program.parse(process.argv);

// Display help if no arguments are provided
if (!process.argv.slice(2).length) {
  program.outputHelp();
}
