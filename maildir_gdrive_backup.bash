# Creates a .tbz archive out of `/var/mail/`, encrypts it, and uploads it to google drive

FILENAME="/tmp/ameoserver_mail_$(date +'%d_%m_%y').tbz"
tar -jcf $FILENAME /var/mail/
gpg --encrypt --cipher-algo CAMELLIA256 --passphrase $PASSWORD -o "$FILENAME.gpg" --quiet $FILENAME
gdrive upload "$FILENAME.gpg"

rm $FILENAME "$FILENAME.gpg"
