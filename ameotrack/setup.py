from setuptools import setup

setup(
    name="ameotrack",
    version="0.1.0",
    packages=["ameotrack"],
    entry_points={"console_scripts": ["ameotrack=ameotrack.__main__:main"]},
    install_requires=[
        "toml~=0.10",
        "requests~=2.20",
        "toml~=0.10",
        "termcolor~=1.1",
        "pyperclip~=1.7",
        "requests-toolbelt"
    ],
    license="MIT",
    url="https://github.com/Ameobea/misc_scripts/tree/master/ameotrack",
    author="Casey Primozic (Ameo)",
    author_email="me@ameo.link",
)
