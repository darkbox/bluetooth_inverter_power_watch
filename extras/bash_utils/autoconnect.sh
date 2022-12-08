#!/usr/bin/expect -f
if {$argc != 2} {
    puts "No arguments."
    exit 1
}

# First argument needs to be the device MAC address;
# ... second argument needs to be the PIN.
set DEV [lindex $argv 0]
set PIN [lindex $argv 1]

# Do not wait after getting a response.
#set timeout -1

# bluetoothctl is in $PATH
spawn bluetoothctl

# Wait for complete startup.
expect "Agent registered"

# If the device is already paired, we remove it first, so we are in a known state of affairs;
# ... we also handle the case in which the device is not already paired.
send "remove $DEV\r"
expect -re "removed|not available"

send "scan on\r"
# Only scan until we see a response from the device;
# ... if the device is not on or in range, this will block the script in scanning;
# ... please make sure the device is: 1) in range, 2) powered on and 3) discoverable.
expect "Device $DEV"

# Stop scanning.
send "scan off\r"
expect "Discovery stopped"

# Request pairing.
send "pair $DEV\r"
expect "Enter PIN code:"

# Send PIN.
send "$PIN\r"
expect "Pairing successful"

sleep 3
send "quit\r"
