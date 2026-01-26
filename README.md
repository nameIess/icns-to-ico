# ICNS to ICO Converter

A simple and user-friendly Python script to batch convert Apple ICNS icon files to Windows ICO format. Perfect for developers and designers working with cross-platform applications.

## Features

- 🚀 **Batch Conversion**: Convert multiple ICNS files at once
- 📁 **Auto Folder Management**: Automatically creates input/output directories
- 🖥️ **GUI Integration**: Opens folders automatically for easy file management
- 🏗️ **Standalone Executable**: Build a single exe file with no dependencies
- ⚡ **Fast Processing**: Uses Pillow library for efficient image processing

## Quick Start

### Option 1: Run the Python Script

```bash
# Install dependencies
pip install pillow

# Run the converter
python convert.py
```

### Option 2: Use the Standalone Executable

Download the latest release from [GitHub Releases](https://github.com/nameIess/icns-to-ico/releases)

```bash
# Simply run the exe
icns-to-ico.exe
```

## How It Works

1. **Directories Created**: The tool automatically creates `icons/icns` and `icons/ico` folders in your Downloads directory
2. **Input Folder Opens**: The `Downloads/icons/icns` folder opens for you to add .icns files
3. **Press Enter** when ready
4. **Conversion Runs**: All .icns files are converted to .ico format
5. **Output Folder Opens**: The `Downloads/icons/ico` folder opens to show your converted files

## Getting ICNS Icons

Download high-quality macOS icons from:

- [macOS Icons](https://macosicons.com/#/) - Free collection of macOS system icons

## Building from Source

### Prerequisites

- Python 3.6+
- Pillow: `pip install pillow`
- PyInstaller (for building exe): `pip install pyinstaller`

### Build Commands

```bash
# Build executable only
python convert.py --build

# Or manually
pyinstaller --onefile convert.py
```

## Project Structure

```
icns-to-ico/
├── convert.py          # Main conversion script
├── README.md           # This file
├── .gitignore         # Git ignore rules
└── icons/             # Working directories (auto-created)
    ├── icns/          # Input: Place .icns files here
    └── ico/           # Output: Converted .ico files appear here
```

## Requirements

- **Python 3.6+** with Pillow library
- **PyInstaller** (optional, for building exe)
- **Windows** (for the executable version)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

[This project is open source. Feel free to use and modify.](LICENSE)
