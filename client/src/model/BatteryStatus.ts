export type BatteryStatus = {
    current_consumed: number;
    energy_consumed: number;
    temperature: number;
    voltages: number[];
    current_battery: number;
    id: number;
    battery_function: number;
    mavtype: number;
    battery_remaining: number;
};