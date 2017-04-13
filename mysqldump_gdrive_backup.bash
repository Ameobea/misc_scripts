# Dumps all databases from the local MySQL database, gzips them, and uploads them to google drive.

FILENAME="/tmp/ameoserver_mysql_$(date +'%d_%m_%y').tgz"                                                                                                                                                                                   
mysqldump --all-databases -p$(DB_PASSWORD) | gzip > $FILENAME && gdrive upload $FILENAME && rm $FILENAME
