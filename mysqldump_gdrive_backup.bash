#!/bin/bash

# Dumps all databases from the local MySQL database, gzips them, and uploads them to google drive.

FILENAME="/tmp/ameoserver_mysql_$(date +'%d_%m_%y').tgz"                                                                                                                                                                                   
mysqldump --all-databases -p$(DB_PASSWORD) | gzip > $FILENAME \
  && gdrive upload -p 0Bw3Lu3S0XCdOMmNFZFVmRV94SFk $FILENAME \
  && rm $FILENAME
