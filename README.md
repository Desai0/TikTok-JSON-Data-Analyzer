# TikTok JSON Data Analyzer

**A deep-dive analysis and visualization tool for your TikTok data export.**

This project is a heavily modified and enhanced version of the original [tiktok_json_analyzer](https://github.com/Elazrod56/tiktok_json_analyzer) by [Elazrod56](https://github.com/Elazrod56). While the core idea remains, this version includes significant architectural improvements, robust error handling, detailed activity reports, and console-based charts for better data visualization.

---

## ✨ Features

- **Detailed Statistics**: Get a comprehensive overview of your activity, including logins, video consumption, likes, comments, and more.
- **First & Last Activity**: See your very first and most recent comments, likes, watched videos, and direct messages.
- **Console Charts**: Visualize your data directly in the terminal with clean, text-based diagrams for:
    - **DM Distribution**: See who you interact with the most.
    - **Daily Time Spent**: Compare your daily time on TikTok with the world average.
- **Robust & Flexible**: Works with the latest TikTok JSON export format and allows you to specify the path to your data file.
- **Cross-Platform**: Built with Rust, it compiles and runs on Windows, macOS, and Linux.

---

## 🚀 Getting Started

### Prerequisites

You need to have the Rust programming language toolchain installed. If you don't have it, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation & Usage

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/Desai0/TikTok-JSON-Data-Analyzer
    cd TJDA
    ```

2.  **Request your TikTok data:**
    - Go to your TikTok settings: `Settings and privacy` -> `Account` -> `Download your data`.
    - Request the data in **JSON format**. This can take a day or two.
    - Once you receive the file, unzip it. You will get a `user_data.json` file.

3.  **Run the analysis:**
    - Place your `user_data.json` file anywhere you like.
    - Run the program using `cargo run`, providing the path to your JSON file as an argument:
    ```sh
    cargo run -- path/to/your/user_data.json
    ```
    - For example, if you place the file in a `json` subfolder:
    ```sh
    cargo run -- json/user_data.json
    ```

4.  **Enjoy your stats!**
    The program will compile and display all your statistics and charts directly in the console.

---


## 🙏 Acknowledgements

- A big thank you to **Elazrod56** for creating the original project that served as the foundation for this version.
