import * as chalk from "chalk";
import { Tests } from "./tests";
import { RandomConfigGenerator } from "./utils/randomConfigGenerator";

type TestResult = {
  testName: string;
  passed: boolean;
};

export class Tester {

  /**
   * Class that contains test functions.
   */
  private _tests: Tests = new Tests();

  /**
   * Class that contains random config generation function.
   */
  private _randomConfigGenerator: RandomConfigGenerator =
    new RandomConfigGenerator();

  /**
   * Array to store test results, which will be displayed later.
   */
  private _testResults: TestResult[] = [];

  /**
   * Runs all the tests for the TypeScript formatter.
   * This method generates a random configuration, runs the relevant tests,
   * and summarizes the results of the tests.
   */
  public runTests(): void {
    const randomConfig = this._randomConfigGenerator.generateRandomConfig();
    console.log(chalk.magenta("Random Config:"), randomConfig);

    this._runTest("Object Type and Assignment Formatting", () =>
      this._tests.objectTypeAndAssignmentFormatting(randomConfig)
    );

    this._runTest("Array Formatting", () =>
      this._tests.arrayFormatting(randomConfig)
    );

    this._runTest("Import Formatting", () =>
      this._tests.importFormatting(randomConfig)
    );

    this._summarizeResults();
  }

  /**
   * Executes a test function, tracks the result, and stores the test's outcome.
   * The result (pass/fail) is logged in the test results for later summarization.
   *
   * @param testName - The name of the test being run.
   * @param testFunction - The test function that returns `true` for success and `false` for failure.
   */
  private _runTest(testName: string, testFunction: () => boolean): void {
    const passed = testFunction();
    this._testResults.push({ testName, passed });
  }

  /**
   * Summarizes the results of all the tests that were run.
   * Logs the total number of tests, the number of succeeded and failed tests,
   * and details about which tests failed (if any).
   */
  private _summarizeResults(): void {
    const totalTests = this._testResults.length;
    const passedTests = this._testResults.filter(
      (result) => result.passed
    ).length;
    const failedTests = totalTests - passedTests;

    passedTests > 0
      ? console.log(chalk.green(`Succeeded Tests: ${passedTests}`))
      : "";

    failedTests > 0
      ? console.log(chalk.red(`Failed Tests: ${failedTests}`))
      : "";
    this._testResults.forEach((testResult, index) => {
      !testResult.passed
        ? console.log(chalk.red(`${index + 1} ${testResult.testName}`))
        : "";
    });
    console.log(chalk.cyan(`Total Tests: ${totalTests}`));
  }
}
