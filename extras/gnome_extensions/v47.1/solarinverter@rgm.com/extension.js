/* extension.js
 *
 * @author      Rafa G.M.
 * @copyright   2023
 * @license     GPL-3.0-only

 */
import St from 'gi://St';
import GLib from 'gi://GLib';
import GObject from 'gi://GObject';
import Soup from 'gi://Soup?version=3.0';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import * as PanelMenu from 'resource:///org/gnome/shell/ui/panelMenu.js';
import * as PopupMenu from 'resource:///org/gnome/shell/ui/popupMenu.js';
import { Extension, gettext as _ } from 'resource:///org/gnome/shell/extensions/extension.js';
const Clutter = imports.gi.Clutter;
const Mainloop = GLib.MainLoop;

const Indicator = GObject.registerClass(
    class Indicator extends PanelMenu.Button {

        inverter_data = JSON.parse('{"model_type":0,"topology":"","cpu_version":"","blt_version":"","ac_voltage":0.0,"ac_frequency":0.0,"pv_input_voltage_stage1":0.0,"pv_input_power_stage1":0,"pv_input_voltage_stage2":0.0,"pv_input_power_stage2":0,"pv_input_voltage_stage3":0.0,"pv_input_power_stage3":0,"pv_input_voltage_stage4":0.0,"pv_input_power_stage4":0,"output_voltage":0.0,"output_frequency":0.0,"output_apparent_power":0,"output_active_power":0,"load_percentage":0,"output_mode":0,"charge_mode":0,"bulk_charge":0,"watts_unkown_01":0,"unkown_02":0.0,"unkown_03":0,"nominal_ac_voltage":0.0,"nominal_ac_current":0.0,"rated_battery_voltage":0.0,"nominal_output_voltage":0.0,"nominal_output_frequency":0.0,"nominal_output_apparent_power":0,"nominal_output_active_power":0,"workmode":"","battery_voltage":0.0,"battery_capacity":0,"battery_charge_current":0,"battery_current_discharge":0,"p_bulk_charging_voltage":0.0,"p_float_charging_voltage":0.0,"p_battery_cutoff_voltage":0.0,"p_battery_equalization_enable":false,"p_rt_activate_battery_equalization":false,"p_equalization_time":0,"p_equalization_period":0,"p_equalization_timeout":0,"p_equalization_voltage":0.0,"p_buzzer_alarm":false,"p_backlight":false,"p_overload_auto_restart":false,"p_overtemp_auto_restart":false,"p_beeps_while_primary_source_interrupt":false,"p_overload_bypass":false,"p_lcd_to_default_after_one_min":false,"p_fault_code_record":false,"p_charger_source_priority":0,"p_output_source_priotrity":0,"p_ac_input_range":0,"p_battery_type":0,"p_output_frequency":0.0,"p_output_voltage":0,"p_back_to_grid_voltage":0.0,"p_max_charging_current":0,"p_max_ac_charging_current":0,"p_back_to_discharge_voltage":0.0}');
        battery_data = {};
        api_host = 'http://192.168.0.192:9999';
        timeout = null;

        _init(context) {
            super._init(0.0, _('Bluetooth Inverter Power Watch'));
            this._http_session = new Soup.Session();
            this._decoder = new TextDecoder();

            this._settings = context.getSettings();

            const self = this;

            // Layout
            this._app_indicator_layout = new St.BoxLayout({
                vertical: false, // Arrange horizontally
            });

            // Icon
            this._indicator_icon = new St.Icon({
                icon_name: 'battery-level-100-symbolic',
                style_class: 'system-status-icon',
            });
            this._app_indicator_layout.add_child(this._indicator_icon);

            // Label
            this._battery_percentage_label = new St.Label({
                text: '\u2026',
                y_expand: true,
                y_align: Clutter.ActorAlign.CENTER
            });
            this._app_indicator_layout.add_child(this._battery_percentage_label);

            // Attach layout
            this.add_child(this._app_indicator_layout);

            this.connect("button-press-event", () => {
                self._requestData();
            });

            this.battery = new PopupMenu.PopupMenuItem(_('Batería'), { reactive: false });
            this.battery_status = new PopupMenu.PopupMenuItem(_('Batería:'), { reactive: false });
            this.solar_voltage = new PopupMenu.PopupMenuItem(_('Voltaje solar'), { reactive: false });
            this.solar_current = new PopupMenu.PopupMenuItem(_('Potencia solar'), { reactive: false });
            this.load_percentage = new PopupMenu.PopupMenuItem(_('Carga inversor'), { reactive: false });
            this.battery_left = new PopupMenu.PopupMenuItem(_('Tiempo restante'), { reactive: false });
            this.menu.addMenuItem(this.load_percentage);
            this.menu.addMenuItem(this.battery);
            this.menu.addMenuItem(this.battery_status);
            this.menu.addMenuItem(this.battery_left);
            this.menu.addMenuItem(this.solar_voltage);
            this.menu.addMenuItem(this.solar_current);

            // let item = new PopupMenu.PopupMenuItem(_('Refrescar'));
            // item.connect('activate', () => {
            //     this._requestData();
            // });
            // this.menu.addMenuItem(item);

            // Request/Refresh data every 10 minutes
            const minutes = 10;
            this._interval_task = setInterval(function () {
                this._requestData();
                this._requestBatteryData();
            }.bind(this), 1000); // minutes * 60 * 1000

            // Update on start
            setTimeout(function () {
                this._requestData();
                this._requestBatteryData();
            }.bind(this), 300);
        }

        customDestroy() {
            if (this._interval_task) {
                clearInterval(this._interval_task);
            }

            this._settings = null;
        }

        _refreshUI() {
            const load_percentage = this.inverter_data.load_percentage;
            this.load_percentage.label.text = _('Carga inversor: ') + load_percentage + '%';

            const battery_capacity = this._get_state_of_charge();
            this.battery.label.text = _('Batería: ') + battery_capacity.toFixed(2) + '%';

            let status = _('en reposo');
            const battery_charge_current = this.inverter_data.battery_charge_current;
            const battery_current_discharge = this.inverter_data.battery_current_discharge;
            if (battery_charge_current > 0.0) {
                status = _('cargando');
            } else if (battery_current_discharge > 0.0) {
                status = _('descargando');
            }
            this.battery_status.label.text = _('Batería: ') + status;

            const battery_left = this._calc_battery_runtime();
            const formatted_time = this._format_hours(battery_left);
            this.battery_left.label.text = _('Tiempo restante: ') + formatted_time;

            const pv_input_voltage_stage1 = this.inverter_data.pv_input_voltage_stage1;
            const pv_input_power_stage1 = this.inverter_data.pv_input_power_stage1;
            this.solar_voltage.label.text = _('Tensión paneles: ') + pv_input_voltage_stage1.toFixed(2) + 'v';
            this.solar_current.label.text = _('Potencia paneles: ') + pv_input_power_stage1 + 'w';

            Mainloop.idle_add(() => { // Update UI on the main thread
                this._updateIndicatorIcon();
                return GLib.SOURCE_REMOVE; // Important to remove the idle source
            });
        }

        _updateIndicatorIcon() {
            const isCharging = (parseInt(this.inverter_data.battery_charge_current) > 0);
            const battery_capacity = Math.round(this._get_state_of_charge());

            // Update indicator icon depending of the current battery capacity
            if (battery_capacity >= 90) {
                this._indicator_icon.icon_name = isCharging ? 'battery-level-100-charging-symbolic' : 'battery-level-100-symbolic';
            } else if (battery_capacity < 90) {
                this._indicator_icon.icon_name = isCharging ? 'battery-level-80-charging-symbolic' : 'battery-level-80-symbolic';
            } else if (battery_capacity < 70) {
                this._indicator_icon.icon_name = isCharging ? 'battery-level-70-charging-symbolic' : 'battery-level-70-symbolic';
            } else if (battery_capacity < 60) {
                this._indicator_icon.icon_name = isCharging ? 'battery-level-60-charging-symbolic' : 'battery-level-60-symbolic';
            } else if (battery_capacity < 50) {
                this._indicator_icon.icon_name = isCharging ? 'battery-level-50-charging-symbolic' : 'battery-level-50-symbolic';
            } else if (battery_capacity < 45) {
                this._indicator_icon.icon_name = isCharging ? 'battery-caution-symbolic' : 'battery-level-0-symbolic';
            } else {
                // Default
                this._indicator_icon.icon_name = 'battery-missing-symbolic';
            }

            // Update label
            this._battery_percentage_label.text = `${battery_capacity}%`;

            // Main.notify('Dev - Battery capacity: ' + battery_capacity + '%');
        }

        _requestData() {
            const endpoint = this.api_host + "/api/info";
            const message = Soup.Message.new('GET', endpoint);

            this._http_session.send_and_read_async(message, GLib.PRIORITY_DEFAULT, null, function (session, res) {
                let data = session.send_and_read_finish(res);
                if (data) {
                    data = this._decoder.decode(data.toArray())
                    this.inverter_data = JSON.parse(data);
                    this._refreshUI();
                }
            }.bind(this));
        }

        _requestBatteryData() {
            const endpoint = this.api_host + "/api/info/battery";
            const message = Soup.Message.new('GET', endpoint);

            this._http_session.send_and_read_async(message, GLib.PRIORITY_DEFAULT, null, function (session, res) {
                let data = session.send_and_read_finish(res);
                if (data) {
                    data = this._decoder.decode(data.toArray())
                    this.battery_data = JSON.parse(data);
                }
            }.bind(this));
        }

        _clamp(num, min, max) {
            return Math.min(Math.max(num, min), max);
        }

        _split_time(number_of_hours) {
            const days = Math.floor(number_of_hours / 24);
            const remainder = number_of_hours % 24;
            const hours = Math.floor(remainder);
            const minutes = Math.floor(60 * (remainder - hours));
            return { days, hours, minutes };
        }

        _format_hours(hours) {
            const time = this._split_time(hours);
            let str = "";

            if (time.days > 0) {
                str += time.days;
                str += (days > 1) ? " días " : " día ";
            }

            str += time.hours + "h ";
            str += time.minutes + "min";
            return str;
        }

        _get_battery_discharge_efficiency_rate() {
            const type = this.inverter_data.p_battery_type;
            if (type == 3 || type == 5) {
                // Lithium (LiFePO4, LiPo, Li-ion, etc.)
                return 0.95;
            }

            // Lead acid (sealed, flooded, AGM, gel)
            return 0.85;
        }

        _get_recommended_depth_of_discharge() {
            const dod = this._clamp(this._settings.get_int('battery-dod'), 0, 100);
            return dod / 100; // example: 0.8 => 80% => 20% left
        }

        _get_inverter_efficiency() {
            // (4% - 90%)
            return Math.abs(0.04 - 0.9);
        }

        _get_state_of_charge() {
            let soc = this.inverter_data.battery_capacity / 100.0;
            if (this.battery_data && Object.hasOwn(this.battery_data, 'soc')) {
                soc = parseFloat(this.battery_data.soc);
            }
            return soc;
        }

        _get_battery_capacity_watts_hours() {
            return this._settings.get_double('battery-wh');
        }

        _calc_battery_runtime() {
            const soc = this._get_state_of_charge();
            const ie = this._get_inverter_efficiency();
            const dod = this._get_recommended_depth_of_discharge();
            const der = this._get_battery_discharge_efficiency_rate();
            const load = this.inverter_data.output_active_power;
            const battery_voltage = this.inverter_data.rated_battery_voltage;
            const battery_capacity = this._get_battery_capacity_watts_hours();
            const battery_ah = battery_capacity / battery_voltage;

            // hours
            return (battery_ah * battery_voltage * der * dod * soc * ie) / load;
        }

        // _parseField = {
        //     model_type: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "Grid tie";
        //             case 1: return "Off grid";
        //         }

        //         return 'Hybrid';
        //     },
        //     p_battery_type: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "AGM";
        //             case 1: return "Flooded";
        //             case 2: return "User define";
        //             case 3: return "Pylon";
        //             case 4: return "WECO";
        //             case 5: return "Other Li battery";
        //         }
        //         return 'Other Li battery';
        //     },
        //     p_charger_source_priority: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "Utility first";
        //             case 1: return "Solar first";
        //             case 2: return "Utility and Solar";
        //             case 3: return "Only Solar Charging";
        //         }
        //         return 'Only Solar Charging';
        //     },
        //     p_ac_input_range: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "Appliance";
        //         }
        //         return "UPS";
        //     },
        //     p_output_source_priotrity: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "USB Priority";
        //             case 1: return "SUB Priority";
        //             case 2: return "SBU Priority";
        //         }
        //         return "SBU Priority";
        //     },
        //     output_mode: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "Single machine output";
        //             case 1: return "Parallel output";
        //             case 2: return "Phase 1 of 3 Phase output";
        //             case 3: return "Phase 2 of 3 Phase output";
        //             case 4: return "Phase 3 of 3 Phase output";
        //         }
        //         return "Phase 3 of 3 Phase output";
        //     },
        //     charge_mode: (value) => {
        //         switch (parseInt(value)) {
        //             case 0: return "Auto";
        //             case 1: return "2-stage";
        //             case 2: return "3-stage";
        //         }
        //         return "3-stage";
        //     },
        // };
    }
);

export default class SIExtension extends Extension {
    enable() {
        this._settings = this.getSettings();
        this._indicator = new Indicator(this);
        Main.panel.addToStatusArea(this.uuid, this._indicator);
    }

    disable() {
        this._settings = null;
        this._indicator.customDestroy();
        this._indicator.destroy();
        this._indicator = null;
    }
}

