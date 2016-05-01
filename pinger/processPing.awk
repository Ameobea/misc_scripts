{
  if($0 ~ "no answer yet"){
    print("-1")
  }else{
    split($0,ping1,"time=")
    ping2=ping1[2]
    split(ping2,ping," ms")
    print(ping[1])
  }
}
