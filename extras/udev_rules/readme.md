# UDEV RULE

For consistent access to USB devices, modify the rule with the device info (Product ID, Vendor ID, Serial number, etc) and place the file into `/etc/udev/rules.d/` directory.
Then reload and restart the udev service:

```bash
sudo service udev reload && sudo service udev restart
```

Or:
```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```


> If symlink **canbus** do not show up, reboot the system.

Now instead of using `/dev/ttyUSBX` on the `.env` file we can just use `/dev/canbus` :)