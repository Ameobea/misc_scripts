# Configure these two lines below before running script:
HOSTIP=ip.ameobea.me # IP of what we're pinging
OUTPUTFILE=./out.txt # Name of the file where results are output
MAXPING=10 # Pings that are higher than this are recorded to file.

while [ 1 -gt 0 ]
do
  RESULT=`ping -D -O -c 1 $HOSTIP | awk -W interactive -f processPing.awk`
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
