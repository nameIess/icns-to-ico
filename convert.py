import os
import sys
import subprocess
from PIL import Image

def create_directories():
    """Create the icons/icns and icons/ico directories in the user's Downloads folder."""
    downloads_path = os.path.join(os.path.expanduser('~/Downloads'), 'temp', 'icons')
    icns_dir = os.path.join(downloads_path, 'icns')
    ico_dir = os.path.join(downloads_path, 'ico')
    os.makedirs(icns_dir, exist_ok=True)
    os.makedirs(ico_dir, exist_ok=True)
    return icns_dir, ico_dir

def convert_icns_to_ico(input_dir, output_dir):
    if not os.path.exists(input_dir):
        print(f"Input directory {input_dir} does not exist.")
        return []

    converted = []
    for filename in os.listdir(input_dir):
        if filename.lower().endswith('.icns'):
            input_path = os.path.join(input_dir, filename)
            output_filename = os.path.splitext(filename)[0] + '.ico'
            output_path = os.path.join(output_dir, output_filename)

            try:
                with Image.open(input_path) as im:
                    im.save(output_path, format='ICO')
                    print(f"Converted: {filename} -> {output_filename}")
                    converted.append(filename)
            except Exception as e:
                print(f"Error converting {filename}: {e}")

    return converted

def build_exe():
    try:
        print("Building executable...")
        subprocess.run([sys.executable, "-m", "PyInstaller", "--onefile", "--clean", __file__], check=True)
        exe_name = os.path.splitext(os.path.basename(__file__))[0] + ".exe"
        exe_path = os.path.join("dist", exe_name)
        if os.path.exists(exe_path):
            final_exe = "icns-to-ico.exe"
            if os.path.exists(final_exe):
                os.remove(final_exe)
            os.rename(exe_path, final_exe)
            print(f"Executable built: {final_exe}")
        else:
            print("Executable not found in dist directory.")
    except subprocess.CalledProcessError as e:
        print(f"Failed to build executable: {e}")
    except FileNotFoundError:
        print("PyInstaller not found. Install it with: pip install pyinstaller")

def main():
    # Check for build flag first
    if len(sys.argv) > 1 and sys.argv[1] == "--build":
        build_exe()
        return

    print("ICNS to ICO Converter")
    print("====================")

    # Check if Pillow is installed
    try:
        import PIL
    except ImportError:
        print("Pillow library not found. Install it with: pip install pillow")
        sys.exit(1)

    # Create directories
    icns_dir, ico_dir = create_directories()
    print(f"Directories created: {icns_dir} and {ico_dir}")

    # Open the icns directory
    try:
        subprocess.run(['explorer', os.path.abspath(icns_dir)], check=False)
    except Exception as e:
        print(f"Could not open directory: {icns_dir} - {e}")

    # Prompt user
    print("\nPlace your .icns files in the 'icons/icns' folder.")
    input("Press Enter when ready to continue...")

    try:
        # Convert files
        converted_files = convert_icns_to_ico(icns_dir, ico_dir)
        num_converted = len(converted_files)
        print(f"\nConversion complete! Converted {num_converted} files.")

        # Preserve originals by renaming directory
        if num_converted > 0:
            backup_base = 'dump-icns'
            parent_dir = os.path.dirname(icns_dir)
            counter = 0
            while True:
                suffix = f"-{counter}" if counter > 0 else ""
                backup_path = os.path.join(parent_dir, f"{backup_base}{suffix}")
                if not os.path.exists(backup_path):
                    break
                counter += 1
            try:
                os.rename(icns_dir, backup_path)
                print(f"Renamed {icns_dir} to {backup_path}")
                os.makedirs(icns_dir, exist_ok=True)
                print(f"Created new {icns_dir}")
            except Exception as e:
                print(f"Failed to preserve originals: {e}. Originals remain in {icns_dir}.")

    except KeyboardInterrupt:
        print("\nConversion interrupted by user. Exiting gracefully.")
        sys.exit(0)

    # Open the ico directory to show results
    if num_converted > 0:
        try:
            subprocess.run(['explorer', os.path.abspath(ico_dir)], check=False)
        except Exception as e:
            print(f"Could not open output directory: {ico_dir} - {e}")

    print("\nTo build an executable, run: python convert.py --build")
    print("Or manually: pyinstaller --onefile convert.py")

if __name__ == "__main__":
    main()