import os

import pandas as pd
from sqlalchemy import create_engine
from sqlalchemy.exc import SQLAlchemyError
from dotenv import load_dotenv


def fetch_updates(engine):
    try:
        with engine.connect() as connection:
            return pd.read_sql("SELECT * FROM updates;", connection)
    except SQLAlchemyError as e:
        print(f"Error fetching data: {e}")


def main():
    load_dotenv()

    DB_HOST = os.getenv("DB_HOST")
    DB_USER = os.getenv("DB_USER")
    DB_PASSWORD = os.getenv("DB_PASSWORD")
    if not DB_HOST or not DB_USER or not DB_PASSWORD:
        print("Database credentials not set")
        return

    DB_NAME = "osutrack_migration"

    FTP_USERNAME = os.getenv("FTP_USERNAME")
    FTP_PASSWORD = os.getenv("FTP_PASSWORD")
    FTP_HOSTNAME = os.getenv("FTP_HOSTNAME")
    if not FTP_USERNAME or not FTP_PASSWORD or not FTP_HOSTNAME:
        print("FTP credentials not set")
        return

    engine = create_engine(
        f"mysql+pymysql://{DB_USER}:{DB_PASSWORD}@{DB_HOST}/{DB_NAME}"
    )

    out_path = "/tmp/osutrack_updates_full.csv.gz"
    try:
        os.remove(out_path)
    except FileNotFoundError:
        pass

    print("Fetching updates...")
    updates = fetch_updates(engine)
    print("Finished fetching updates. Dumping to CSV...")
    updates.to_csv(out_path, index=False, compression="gzip", header=True)

    print("Uploading to FTP...")
    cmd = f'curl -T "{out_path}" ftp://{FTP_USERNAME}:{FTP_PASSWORD}@{FTP_HOSTNAME}/osutrack_updates_full.csv.gz'
    os.system(cmd)

    os.remove(out_path)


if __name__ == "__main__":
    main()
