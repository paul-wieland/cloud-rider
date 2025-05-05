import {BatteryStatus} from "@/model/BatteryStatus.ts";
import {Card, CardContent} from "@/components/ui/card.tsx";


export const BatteryStatusCard: React.FC<{ battery: BatteryStatus | null }> = ({ battery }) => {
    const tempC = battery?.temperature != null ? (battery.temperature / 100).toFixed(1) : "-";
    const currentMa = battery?.current_battery != null ? battery.current_battery * 10 : "-";
    const batteryPct =
        battery?.battery_remaining != null && battery.battery_remaining >= 0
            ? `${battery.battery_remaining}%`
            : "-";

    return (
        <Card className="bg-black/80 text-white text-sm px-3 py-2 border-none backdrop-blur-sm">
            <CardContent className="p-0 space-y-1 text-left">
                <p>Battery: {batteryPct}</p>
                <p>Temp: {tempC} °C</p>
                <p>Current: {typeof currentMa === "number" ? `${currentMa} mA` : "-"}</p>
            </CardContent>
        </Card>
    );
};