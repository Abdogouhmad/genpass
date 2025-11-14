# GenPass: Password Generator CLI

GenPass is a command-line interface (CLI)
tool built with Rust for generating secure passwords.
It allows you to create strong, random passwords with various customization options,
including length and the inclusion of numbers, symbols, and uppercase letters.
You can also add a note to each generated password
and save the credentials to a text file for your records.

## Features

- **Secure Password Generation:** Create strong,
random passwords of customizable length.
- **Customizable Password Complexity:**
  - Include or exclude numbers.
  - Include or exclude symbols.
  - Include or exclude uppercase letters.
- **Save with Notes:** Store your generated passwords along
with a descriptive note in a `.txt` file.
- **Easy to Use:** A simple and intuitive command-line interface.

## Installation

1. **Prerequisites:**
   - Ensure you have Rust installed on your system.
   If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

2. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-username/genpass.git
   cd genpass
   ```

3. **Build the Project:**

   ```bash
   cargo build --release
   ```

   The executable will be located in the `target/release/` directory.

## Usage

To use GenPass, you can run the executable with
various optional flags to customize your password.

### Basic Usage

To generate a password with default settings, run:

```bash
genpass
```

### Customization Options

You can customize the generated password using the following optional flags:

- `--usr`: Specify a username.
- `--password`: Generate a password.
- `--note <your-note>`: Add a note to the saved credentials.

### Examples

- **Generate a password with a username and a note:**

  ```bash
  genpass --usr myuser --password --note "For my Instagram account"
  ```

- **Generate a password with default settings and a note:**

  ```bash
  genpass --password --note "For my Google account"
  ```

By default, all generated credentials will be saved in a `.txt` file in the project directory.

## Contributing

Contributions are welcome! If you have ideas for new features, improvements, or bug fixes, feel free to open an issue or submit a pull request.

1. **Fork the repository.**
2. **Create a new branch:** `git checkout -b feature/your-feature-name`
3. **Make your changes and commit them:** `git commit -m 'Add some feature'`
4. **Push to the branch:** `git push origin feature/your-feature-name`
5. **Submit a pull request.**

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
