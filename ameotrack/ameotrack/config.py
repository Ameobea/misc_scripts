import os
import pathlib

import toml

DEFAULT_CONF = {"ameotrack_url_root": "https://ameo.link/u", "upload_password": "pass"}
CONFIG_DIR_PATH = os.path.join(os.path.expanduser("~"), ".ameotrack")
CONF_FILE_PATH = os.path.join(CONFIG_DIR_PATH, "conf.toml")


def init_config():
    pathlib.Path(CONFIG_DIR_PATH).mkdir(parents=False, exist_ok=True)

    # Initialize config file with default config if it's empty
    if not os.path.isfile(CONF_FILE_PATH):
        default_conf_toml = toml.dumps(DEFAULT_CONF)
        with open(CONF_FILE_PATH, "w") as f:
            f.write(default_conf_toml)

    return open(CONF_FILE_PATH, "r")


def load_conf(conf_file) -> dict:
    conf_file = conf_file or init_config()
    conf_toml = conf_file.read()
    conf_file.close()

    try:
        return toml.loads(conf_toml)
    except toml.TomlDecodeError:
        print("Error reading supplied config file!")

    exit(1)
