import {BatteryStatus} from "@/model/BatteryStatus.ts";
import {Card, CardContent} from "@/components/ui/card.tsx";


export const BatteryStatusCard: React.FC<{ battery: BatteryStatus | null }> = ({ battery }) => {
    const tempC = battery?.temperature_c != null ? (battery.temperature_c / 100).toFixed(1) : "-";
    const currentMa = battery?.current_battery_ma != null ? battery.current_battery_ma * 10 : "-";
    const batteryPct =
        battery?.remaining_percent != null && battery.remaining_percent >= 0
            ? `${battery.remaining_percent}%`
            : "-";

    return (
        <Card className="bg-[rgba(0,0,0,0.8)] text-white text-sm px-3 py-2 border-none backdrop-blur-sm">
            <CardContent className="p-0 space-y-1 text-left">
                <p>Battery: {batteryPct}</p>
                <p>Temp: {tempC} °C</p>
                <p>Current: {typeof currentMa === "number" ? `${currentMa} mA` : "-"}</p>
            </CardContent>
        </Card>
    );
};