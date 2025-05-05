import React from "react";
import {
    Card,
    CardContent,
} from "@/components/ui/card";
import {GlobalPosition} from "@/model/GlobalPosition.ts";
import {Heartbeat} from "@/model/Heartbeat.ts";



type TelemetryCardProps = {
    telemetry?: GlobalPosition | null;
    heartbeat?: Heartbeat | null;
};

export const TelemetryCard: React.FC<TelemetryCardProps> = ({ telemetry, heartbeat }) => {
    return (
        <Card className="bg-black/80 p-2 border-none backdrop-blur-sm">
            <CardContent className="text-left pt-1">
                    <div>
                        <h2 className="font-medium text-white mb-2">Global Position</h2>
                        <ul className="text-sm text-white">
                            <li><span className="font-medium">Latitude:</span> {telemetry?.lat.toFixed(6) ?? "-"}</li>
                            <li><span className="font-medium">Longitude:</span> {telemetry?.lon.toFixed(6) ?? "-"}</li>
                            <li><span className="font-medium">Altitude:</span> {telemetry?.alt.toFixed(2) ?? "-"} </li>
                            <li><span className="font-medium">Velocity X:</span> {telemetry?.vx?.toFixed(2) ?? "-"} m/s</li>
                            <li><span className="font-medium">Velocity Y:</span> {telemetry?.vy?.toFixed(2) ?? "-"} m/s</li>
                            <li><span className="font-medium">Velocity Z:</span> {telemetry?.vz?.toFixed(2) ?? "-"} m/s</li>
                        </ul>
                            <p className="text-xs text-gray-300 mt-2">
                                Last heartbeat: { heartbeat?.timestamp ? new Date(heartbeat.timestamp).toLocaleTimeString() : "-"}
                            </p>
                    </div>
            </CardContent>
        </Card>
    );
};
