# Script that generates a timestamped proof of a file that can be used to guarentee
# that it existed at a certain point in time.
#
# The .asc file is the signature proving that you created `file` and the .gpg file
# is an encrypted copy of the file along with some random data to prevent brute forcing.

if [ $# -eq 0 ]
  then
    echo "usage: ./prove.sh file"
    exit 1
fi

cat $1 > $1.tmp
echo "------------------------------------
The following is random text to prevent brute-forcing signature checks:" >> $1.tmp
cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 64 | head -n 1 >> $1.tmp

# Create an ASCII-armored, detached signature of the file
gpg -b -a -o $1.asc $1.tmp

clear
echo "Enter a password that will decrypt the original document"
# Create an encrypted version of the proof
gpg --symmetric --cipher-algo CAMELLIA256 -o $1.gpg $1.tmp

# Securely delete the temp file
shred -u $1.tmp

clear
echo "Signature generated at ${1}.asc

Encrypted original generated at ${1}.gpg"