To compile schemas:
```bash
glib-compile-schemas schemas/
```

Reset Shell in order to apply changes.
`alt`+`f2` and type `r` in X11 session.

Now in the terminal we can enable or disable the extension with the commands:
```bash
gnome-extensions enable solarinverter@rgm.com
```
```bash
gnome-extensions disable solarinverter@rgm.com
```

Check Gnome Shell log:
```bash
journalctl -f -o cat /usr/bin/gnome-shell
```

Check Prefs log:
```bash
journalctl -f -o cat /usr/bin/gjs
```

More info at: https://gjs.guide/extensions/development/