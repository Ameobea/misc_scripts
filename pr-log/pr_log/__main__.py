import re
import subprocess

import click


COMMIT_HEADER_REGEX = re.compile("commit .+\\nAuthor: .*\\nDate: .*\\n\\n")

is_empty = lambda elem: len(elem) > 0


def read_git_log(target_branch: str) -> str:
    try:
        return subprocess.check_output(f"git log {target_branch}..", shell=True).decode("utf-8")
    except subprocess.CalledProcessError:
        print("Error executing `git log` command.  Are you in a git repository?")
        exit(1)


def deindent_commit_body(body: str) -> str:
    # Trim off the first 4 characters (all spaces) from each line
    deindented_lines = [line[4:] for line in body.split("\n")]
    return "\n".join(filter(is_empty, deindented_lines))


@click.command()
@click.option("--target-branch", "-t", type=click.STRING, default="master")
def main(target_branch):
    log_output = read_git_log(target_branch)
    commit_bodies = filter(is_empty, re.split(COMMIT_HEADER_REGEX, log_output))
    deindented_commit_bodies = [
        deindent_commit_body(commit_body) for commit_body in reversed(list(commit_bodies))
    ]

    print("\n\n".join(deindented_commit_bodies))


main()  # pylint: disable=E1120
