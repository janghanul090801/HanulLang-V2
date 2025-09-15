import sys
from hanul.runtime import janghanul  
from enum import Enum
from importlib.metadata import version, PackageNotFoundError
from rich.console import Console
from rich.markdown import Markdown
from importlib.resources import files
import subprocess

class CliCommand(Enum):
    RUN = 'run'
    VERSION = 'version'
    DIMI = 'dimi'           # 탈락 출력하고 종료
    HELP = 'help'
    DOC = 'doc'     # README.md 출력
    KILL = 'kill'   # pip uninstall hanul


def main():
    if len(sys.argv) == 1:
        print("[commands]")
        for c in list(CliCommand):
            print("\t"+c.value)
        sys.exit(1)

    command = sys.argv[1]
    
    match command:
        case CliCommand.RUN.value:
            filename = sys.argv[2]
            if not filename.endswith(".eagen"):
                print("확장자는 .eagen으로 되어야함")
                sys.exit(1)

            with open(filename, "r", encoding="utf-8") as file:
                code = file.read()

            interpreter = janghanul()
            interpreter.compile(code)
        case CliCommand.VERSION.value:
            try:
                pkg_version = version("hanul")
            except PackageNotFoundError:
                pkg_version = "unknown"

            print(f"Hanul version {pkg_version}")
        case CliCommand.DIMI.value:
            print("떨")
            sys.exit(1)
        case CliCommand.HELP.value:
            pass
        case CliCommand.DOC.value:
            console = Console()
            readme_text = files("hanul").joinpath("README.md").read_text(encoding="utf-8")
            console.print(Markdown(readme_text))
        case CliCommand.KILL.value:
            package_name = "hanul"

            result = subprocess.run([sys.executable, "-m", "pip", "uninstall", package_name, "-y"],
                                    capture_output=True, text=True)

            print(result.stdout)
            print(result.stderr)

        case _:
            print("unknown command")
            sys.exit(1)

if __name__ == '__main__':
    main()