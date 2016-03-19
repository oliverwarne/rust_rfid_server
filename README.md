# Rust Door Server

### What is it?
----
This is the server/backend side of a door management system. A user scans their RFID
card into a scanner built into a door, and it sends the data to this server and the
scanner receives a response indicating if the door should open or not. This part is
the easy side.

### How do I use it?
----
You don't. It's being developed and doesn't do anything right now.

### TODO
----
* Get a multithreaded server working.
* Make everything idiomatic (currently KeyStore is kind of idiomatic)
* Address individual devices via MAC adressess over telnet
* everything