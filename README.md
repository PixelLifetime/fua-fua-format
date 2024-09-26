# Fua Fua Format

**"Fua Fua"** means "Fluffy" in Japanese, symbolizing the flexibility and softness of the formatting process. **Fua Fua Format** is a highly customizable code formatter, allowing you to adjust and fine-tune your code styling exactly to your preferences.

## Features

- Fully configurable code styling.
- Supports multiple file types (e.g., `.ts`, `.html`).
- Easy-to-use CLI for formatting and testing.

## How to Use the Formatter Manually

Follow these steps to format your files using **Fua Fua Format**:

### 1. Open the Project Folder in the Console

First, open a terminal and navigate to your project folder. Change the directory to the `src` folder by running:

```bash
cd C:\Users\User\formatter\src
```

(Adjust the path according to your system's directory structure if needed.)

### 2. Add Files to Format

Place the files you want to format inside the `filesForFormatting` folder. This folder is located in the `src` directory of your project.

### 3. Run the Formatter

To format the files, execute the following command in your terminal:

```bash
ts-node ./cli.ts format --pattern "/**/*.{html,ts}"
```

- `"/**/*.{html,ts}"` is the default pattern, which targets all `.ts` and `.html` files in the `filesForFormatting` directory and its subdirectories.
- You can change the pattern to fit your needs, targeting specific file types or directories.

### 4. View the Formatting Results

After running the command, you'll see the names of the formatted files in the console. If everything is successful, you will get a confirmation message showing the files that were formatted.

---

## Running Tests

You can also run tests to verify the formatter’s functionality. Here’s how:

### 1. Open the Project Folder in the Console

Navigate to the `src` folder in your terminal:

```bash
cd C:\Users\User\formatter\src
```

### 2. Run the Test Command

Run the following command to execute the tests:

```bash
ts-node ./cli.ts test
```

### 3. View Test Results

After the tests are completed, you’ll see a summary of the results in the console:
- The total number of tests.
- The number of tests that succeeded.
- The number of tests that failed, if any, along with details.

---

## Customizing the Formatter

One of the core strengths of **Fua Fua Format** is its flexibility. You can customize the formatting configuration to match your specific coding style. Simply modify the settings in the configuration file (`config.json`), located in the project root, to adjust:
- Indentation
- Spacing
- Object formatting
- And other style preferences

---

## Example

```bash
# Format files in filesForFormatting folder
ts-node ./cli.ts format --pattern "/**/*.{html,ts}"

# Run tests
ts-node ./cli.ts test
```

---

## License

This project is licensed under the MIT License.

---

By following these steps, you’ll be able to effortlessly format your code and run tests using **Fua Fua Format**. Enjoy your beautifully formatted, fluffy code!
