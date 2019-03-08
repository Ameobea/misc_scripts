from setuptools import setup, find_packages

setup(
    name="pr-log",
    version="0.1.0",
    packages=find_packages(),
    entry_points={"console_scripts": ["pr-log=pr_log.__main__:main"]},
    install_requires=["click"],
    license="MIT",
    url="http://github.com/ameobea/misc-scripts",
    author="Casey Primozic (Ameo)",
    author_email="me@ameo.link",
)
