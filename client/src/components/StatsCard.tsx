import React from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Stats } from "@/model/Stats.ts";

function formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    const kb = bytes / 1024;
    if (kb < 1024) return `${kb.toFixed(2)} KB`;
    const mb = kb / 1024;
    if (mb < 1024) return `${mb.toFixed(2)} MB`;
    const gb = mb / 1024;
    return `${gb.toFixed(2)} GB`;
}

export const StatsCard: React.FC<{ stats?: Stats | null }> = ({ stats }) => {
    const messageCount = stats?.messageCount ?? "-";
    const size = stats ? formatBytes(stats.totalBytes) : "-";
    const time = stats
        ? new Date(stats.elapsedSeconds * 1000).toISOString().substr(11, 8)
        : "-";

    return (
        <Card className="bg-[rgba(0,0,0,0.8)] text-white text-sm px-3 py-2 border-none backdrop-blur-sm">
            <CardContent className="p-0 space-y-1 text-left">
                <p>Messages: {messageCount}</p>
                <p>Size: {size}</p>
                <p>Uptime: {time}</p>
            </CardContent>
        </Card>
    );
};
