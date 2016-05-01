# Configure these two lines below before running script:
HOSTIP=67.186.97.226 # IP of what we're pinging
OUTPUTFILE=./out.txt # Name of the file where results are output
MAXPING=200 # Pings that are higher than this are recorded to file.

while [ 1 -gt 0 ]
do
  RESULT=`ping -D -O -c 1 -W 3 $HOSTIP | awk -f processPing.awk | xargs`
  DATE=`date`

  if [ "$RESULT" = "-1" ]
    then
    echo "Unable to reach host at $DATE"
  else
    COMP=$(echo "$RESULT>$MAXPING" | bc -l)

    if [ "$COMP" -gt "0" ]
      then
      echo "Ping was $RESULT at $DATE"
    fi
  fi

  sleep 1s
done
