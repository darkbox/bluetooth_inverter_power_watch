/**
 * Preferences window
 */
import Adw from 'gi://Adw';
import Gio from 'gi://Gio';
import Gtk from 'gi://Gtk';
import { ExtensionPreferences, gettext as _ } from 'resource:///org/gnome/Shell/Extensions/js/extensions/prefs.js';

export default class SIExtensionPreferences extends ExtensionPreferences {
    fillPreferencesWindow(window) {
        const settings = this.getSettings();

        // Create a preferences page, with a single group
        const page = new Adw.PreferencesPage();
        window.add(page);

        const group = new Adw.PreferencesGroup();
        page.add(group);

        // Create a new preferences row
        const row = new Adw.EntryRow({
            title: _('Batería Wh'),
            text: settings.get_double('battery-wh').toString()
        });
        row.set_input_purpose(Gtk.InputPurpose.GTK_INPUT_PURPOSE_NUMBER);
        row.set_show_apply_button(true);
        row.connect('apply', function () {
            let value = parseFloat(row.text);
            settings.set_double('battery-wh', value);
        });
        group.add(row);

        // settings.bind('battery-wh', row, 'text', Gio.SettingsBindFlags.DEFAULT);

        // const row2 = new Adw.EntryRow({
        //     title: _('Batería profundidad de descarga %'),
        //     text: settings.get_int('battery-dod').toString()
        // });
        // group.add(row2);

        const dodButton = new Gtk.SpinButton();
        dodButton.set_sensitive(true);
        dodButton.set_range(0, 100);
        dodButton.set_value(settings.get_int('battery-dod'))
        dodButton.set_increments(1, 2);
        dodButton.connect('value-changed', function (w) {
            settings.set_int('battery-dod', w.get_value_as_int());
        });

        const row4 = new Adw.ActionRow({
            title: _('Batería profundidad de descarga (%)'),
        });
        row4.add_suffix(dodButton);
        group.add(row4);

        // settings.bind('battery-dod', row2, 'text', Gio.SettingsBindFlags.DEFAULT);

        const row3 = new Adw.EntryRow({
            title: _('Host'),
            text: settings.get_string('host')
        });
        group.add(row3);

        settings.bind('host', row3, 'text', Gio.SettingsBindFlags.DEFAULT);


        // Create a switch and bind its value to the `show-indicator` key
        // const toggle = new Gtk.Switch({
        //     active: settings.get_boolean('show-indicator'),
        //     valign: Gtk.Align.CENTER,
        // });
        // settings.bind('show-indicator', toggle, 'active', Gio.SettingsBindFlags.DEFAULT);

        // // Add the switch to the row
        // row.add_suffix(toggle);
        // row.activatable_widget = toggle;

        // Make sure the window doesn't outlive the settings object
        window._settings = settings;
    }
}