#!/bin/bash

# Run daily at 4:30 AM
# 30 4 * * * backup_mongo

ARCHIVE_NAME="robintrack_mongodb_$(date +'%d-%m-%y').archive"
docker exec -it mongo mongodump --db robinhood --archive=/var/${ARCHIVE_NAME}
docker cp mongo:/var/${ARCHIVE_NAME} /tmp/${ARCHIVE_NAME}
docker exec -it mongo rm /var/${ARCHIVE_NAME}
gdrive upload -p 0Bw3Lu3S0XCdOMmNFZFVmRV94SFk /tmp/${ARCHIVE_NAME}
rm /tmp/${ARCHIVE_NAME}
