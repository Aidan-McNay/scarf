# =======================================================================
# conf.py
# =======================================================================
# Documentation configurations

import os
import sys
from pathlib import Path
from datetime import date
import subprocess

if sys.version_info >= (3, 11):
    import tomllib
else:
    import tomli as tomllib  # Fallback for Python < 3.11

# -----------------------------------------------------------------------
# General Project Information
# -----------------------------------------------------------------------
# Parse from the `pyproject.toml`

pyproject_path = Path(__file__).parent.parent / "pyproject.toml"
with open(pyproject_path, "rb") as f:
    pyproject_data = tomllib.load(f)

# Extract standard PEP 621 metadata fields
project_meta = pyproject_data.get("project", {})

# Assign Sphinx info variables based on metadata
project = project_meta.get("name", "scarf_python")
version = project_meta.get("version", "0.1.0")
release = version

authors_list = project_meta.get("authors", [])
author = (
    ", ".join([a.get("name", "") for a in authors_list if "name" in a])
    or "Unknown"
)
copyright = f"{date.today().year}, {author}"


# -----------------------------------------------------------------------
# General Configuration
# -----------------------------------------------------------------------

extensions = ["sphinx_rtd_theme"]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# -----------------------------------------------------------------------
# HTML Output
# -----------------------------------------------------------------------

html_context = {
    "display_github": True,  # Integrate GitHub
    "github_user": "Aidan-McNay",  # Username/Organization
    "github_repo": "scarf",  # Repo name
    "github_version": "main",  # Version
    "conf_py_path": "/scarf-python/docs/",  # Path in the checkout to the docs root
}

html_theme = "sphinx_rtd_theme"
html_static_path = ["_static"]

# -----------------------------------------------------------------------
# Setup Hooks
# -----------------------------------------------------------------------
# Check that the build directory is as intended


def setup(app):
    pass
