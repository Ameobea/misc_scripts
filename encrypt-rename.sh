# Encrypts all files in the target directory, writing encrypted versions of them to
# a destination directory with random filenames.  Creates an index file mapping the
# original filenames to the obuscated ones.  Then encrypts the index file.

if [ $# -eq 0 ]
  then
    echo "usage: ./encrypt.sh target_dir password"
    exit 1
fi

binaries=$(ls $1)
touch $1/index.txt

for f in $binaries; do
    random_filename=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 12 | head -n 1)
    gpg --symmetric --cipher-algo CAMELLIA256 --passphrase $2 -o $1/$random_filename.dat $1/$f
    echo "${random_filename}.dat - $f" >> $1/index.txt
done

gpg --symmetric --cipher-algo CAMELLIA256 --passphrase $2 -o $1/index.dat $1/index.txt
