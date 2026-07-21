from pathlib import Path

def generate_readme():
    root_dir = Path(__file__).parent.parent
    
    content = """# Abyss Dotfiles

**Dotfiles** is a symlink-based Dotfiles manager. 

## Features
* **JSON Output**: Output uses a strict JSON contract and is easy to parse.
"""

    (root_dir / "README.md").write_text(content.lstrip(), encoding="utf-8")

if __name__ == "__main__":
    generate_readme()
