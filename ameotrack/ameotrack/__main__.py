import traceback
from pathlib import Path
from datetime import datetime, timedelta
import json
import re
from functools import partial

import click
import requests
from termcolor import colored
import pyperclip

from .config import load_conf, CONF_FILE_PATH

STATE = None


def print_help():
    with click.Context(command) as ctx:
        click.echo(command.get_help(ctx))
    exit(0)


def print_color(color: str, prefix: str, msg: str):
    print("{} {}".format(colored(prefix, color), msg))


print_err = partial(print_color, "red")

print_success = partial(print_color, "green")


# Taken from https://stackoverflow.com/a/13756038/3833068
def td_format(td_object: timedelta):
    seconds = int(td_object.total_seconds())
    periods = [
        ("year", 60 * 60 * 24 * 365),
        ("month", 60 * 60 * 24 * 30),
        ("day", 60 * 60 * 24),
        ("hour", 60 * 60),
        ("minute", 60),
        ("second", 1),
    ]

    strings = []
    for period_name, period_seconds in periods:
        if seconds > period_seconds:
            period_value, seconds = divmod(seconds, period_seconds)
            has_s = "s" if period_value > 1 else ""
            strings.append("%s %s%s" % (period_value, period_name, has_s))

    return ", ".join(strings)


class AppState(object):
    def __init__(self, conf_file):
        self.conf = load_conf(conf_file)

    def make_request(
        self, method: str, *args, params={}, json_body=None, form_data=None, multipart_data=None
    ):
        (func, kwargs) = {
            "POST": (
                requests.post,
                {"json": json_body, "files": multipart_data, "data": form_data},
            ),
            "GET": (requests.get, {}),
            "PUT": (requests.put, {"json": json_body}),
            "PATCH": (requests.patch, {"json": json_body}),
            "DELETE": (requests.delete, {}),
        }[method.upper()]

        return func(*args, timeout=20.0, params=params, **kwargs)

    def api_call(self, resource_path: str, method="GET", **kwargs):
        try:
            url = "{}/{}".format(self.conf["ameotrack_url_root"], resource_path)
            return self.make_request(method, url, **kwargs).text
        except Exception:
            traceback.print_exc()


class Commands(click.Group):
    def list_commands(self, ctx):
        return ["upload", "remind", "bin"]

    def get_command(self, ctx, cmd_name):
        if not cmd_name:
            print_help()

        commands = self.list_commands(ctx)
        matches = [x for x in commands if x.startswith(cmd_name)]
        if not matches:
            return None
        elif len(matches) == 1:
            return click.Group.get_command(self, ctx, matches[0])
        ctx.fail("Too many matches: %s" % ", ".join(sorted(matches)))


@click.command("ameotrack", cls=Commands)
@click.option(
    "--config",
    "-c",
    type=click.File(encoding="utf-8"),
    default=None,
    help=(
        "Provide an alternative configuration file to use instead of the default at "
        "`~/.ameotrack/conf.toml`"
    ),
)
def main(config):
    global STATE
    STATE = AppState(config)


@main.command()
@click.option("--one-time", "-o", is_flag=True, default=False)
@click.option("--private", "-p", is_flag=True, default=False)
@click.option("--expiry", "-e", type=click.IntRange(1, None))
@click.argument("filename", type=click.Path(exists=True))
def upload(one_time, private, expiry, filename: Path):
    with open(filename, "rb") as f:
        url = STATE.api_call(
            "upload",
            method="POST",
            multipart_data={"file": (filename, f)},
            form_data={
                "expiry": str(expiry or -1),
                "secret": "1" if private else "",
                "oneTime": "1" if one_time else "",
                "password": STATE.conf["upload_password"],
                "source": "at-cli",
            },
        )

        pyperclip.copy(url)
        print("{} {}".format(colored("File successfully uploaded:", "green"), url))
        print("Link has been copied to the clipboard.")


@main.command()
@click.option(
    "--timestamp",
    "-t",
    help="Include a timestamp at the end of the reminder of when the reminder was set",
    is_flag=True,
    default=False,
)
@click.argument("date", type=click.STRING)
@click.argument("message", type=click.STRING)
def remind(timestamp, date, message):
    if timestamp:
        time_str = datetime.now().strftime("%Y-%m-%d %X")
        suffix = f"\nSent at {time_str}"
        message += suffix

    res = STATE.api_call("remind", params={"dateString": date, "message": message})
    try:
        res = json.loads(res)
        if not res["success"]:
            print_err("Error creating reminder:", res.get("reason"))
            return

        delivery_dt = datetime.utcfromtimestamp(res["timestamp"])
        tdelta = delivery_dt - datetime.utcnow()
        tdelta_fmt = td_format(tdelta)
        print_success("Reminder successfully created; will be sent in:", tdelta_fmt)
    except json.JSONDecodeError:
        print_err("Received bad response from server:", res)


@main.command(name="bin")
@click.option(
    "--secret",
    "-s",
    is_flag=True,
    default=False,
    help="Use a long, unguessable URL for the generated bin",
)
@click.argument("password", type=click.STRING)
@click.argument("file_path", type=click.Path(exists=True, dir_okay=False))
def create_bin(password: str, file_path, secret: bool):
    text = ""
    filename = click.format_filename(file_path, shorten=True)

    with open(file_path, "r", encoding="utf-8") as f:
        text = f.read()

    form_data = {"filename": filename, "password": password, "text": text}
    if secret:
        form_data["secret"] = "1"

    res = STATE.api_call("bin", method="POST", form_data=form_data)
    bin_name_rgx = re.compile('; url=\.(.*)"')
    match = bin_name_rgx.search(res)
    if not match:
        print_err("Received bad response from server:", res)
        exit(1)

    bin_url = "{}{}".format(STATE.conf["ameotrack_url_root"], match.groups()[0])
    pyperclip.copy(bin_url)
    print_success("Bin successfully created:", bin_url)
    print("Link has been copied to the clipboard.")


# TODO: List reminders, delete/modify images, phost integration(?)
if __name__ == "__main__":
    main()

