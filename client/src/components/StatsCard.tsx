import React from "react";
import { Card, CardContent } from "@/components/ui/card";
import {Stats} from "@/model/Stats.ts";


export const StatsCard: React.FC<{ stats?: Stats | null }> = ({ stats }) => {
    const messageCount = stats?.messageCount ?? "-";
    const gb = stats ? (stats.totalBytes / (1024 ** 3)).toFixed(3) : "-";
    const time = stats
        ? new Date(stats.elapsedSeconds * 1000).toISOString().substr(11, 8)
        : "-";

    return (
        <Card className="bg-black/80 text-white text-sm px-3 py-2 border-none backdrop-blur-sm">
            <CardContent className="p-0 space-y-1 text-left">
                <p>Messages: {messageCount}</p>
                <p>Size: {gb} GB</p>
                <p>Uptime: {time}</p>
            </CardContent>
        </Card>
    );
};
