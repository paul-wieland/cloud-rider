import './App.css'
import { Button } from "@/components/ui/button"
import {
    Card,
} from "@/components/ui/card"

import { MapContainer } from 'react-leaflet/MapContainer'
import { TileLayer } from 'react-leaflet/TileLayer'
import { Marker } from 'react-leaflet/Marker'
import { Popup } from 'react-leaflet/Popup'
import { useMap } from 'react-leaflet/hooks'

import logo from "/cloud_rider_logo.png"
import { useEffect, useState } from 'react';
import L from 'leaflet'
import {TelemetryCard} from "@/components/TelemetryCard.tsx";
import {BatteryStatusCard} from "@/components/BatteryStatusCard.tsx";
import {BatteryStatus} from "@/model/BatteryStatus.ts";
import {StatsCard} from "@/components/StatsCard.tsx";
import {Stats} from "@/model/Stats.ts";

type Telemetry = {
    lat: number;
    lon: number;
    alt: number;
    relative_alt: number;
    vx: number;
    vy: number;
    vz: number;
};

type Heartbeat = {
    timestamp: string;
};

type Message =
    | { type: "telemetry"; data: Telemetry }
    | { type: "heartbeat"; data: Heartbeat }
    | { type: "battery"; data: BatteryStatus };

function App() {
    const [telemetry, setTelemetry] = useState<Telemetry | null>(null);
    const [heartbeat, setHeartbeat] = useState<Heartbeat | null>(null);
    const [battery, setBattery] = useState<BatteryStatus | null>(null);
    const [stats, setStats] = useState<Stats | null>({messageCount: 0, totalBytes: 0, elapsedSeconds: 0});
    const [isFollowing, setIsFollowing] = useState<boolean>(true);

    const toggleFollow = () => {
        setIsFollowing((prev) => !prev);
    };

    const customIcon = new L.Icon({
        iconUrl: "/drone.png",
        iconSize: [64, 64],
        iconAnchor: [16, 32],
        popupAnchor: [0, -32],
    });

    useEffect(() => {
        const interval = setInterval(() => {
            setStats((prev) => {
                if (!prev) return null;
                return {
                    ...prev,
                    elapsedSeconds: prev.elapsedSeconds + 1,
                };
            });
        }, 1000);
        return () => clearInterval(interval);
    }, []);


    useEffect(() => {
        const socket = new WebSocket('ws://127.0.0.1:3000/ws');

        socket.onmessage = (event) => {
            try {
                const message: Message = JSON.parse(event.data);

                setStats((prev) => {
                    if (!prev) return null;
                    return {
                        ...prev,
                        messageCount: prev.messageCount + 1,
                        totalBytes: prev.totalBytes + event.data.length,
                    };
                });

                switch (message.type) {
                    case "telemetry":
                        setTelemetry(message.data);
                        break;
                    case "heartbeat":
                        setHeartbeat(message.data);
                        break;
                    case "battery":
                        setBattery(message.data);
                        break;
                    default:
                        console.warn("Unknown message type:", message);
                }
            } catch (e) {
                console.error("Failed to parse WebSocket message:", e);
            }
        };

        socket.onerror = (error) => {
            console.error("WebSocket error:", error);
        };

        return () => {
            socket.close();
        };
    }, []);

    return (
        <div className="flex flex-col h-screen">
            <header className="w-full bg-black shadow flex justify-center items-center">
                <img src={logo} alt="Logo" className="h-20" />
            </header>

                <div className="w-full h-full relative">
                    <MapContainer
                        center={[telemetry?.lat ?? 48, telemetry?.lon ?? 11]}
                        zoom={13}
                        scrollWheelZoom={true}
                        className="h-full w-full"
                    >
                        <TileLayer
                            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                        />
                        {telemetry && <MapUpdater telemetry={telemetry} />}
                        <Marker position={[telemetry?.lat ?? 48, telemetry?.lon ?? 11]} icon={customIcon}>
                            <Popup>
                                A pretty CSS3 popup. <br /> Easily customizable.
                            </Popup>
                        </Marker>
                    </MapContainer>

                    <div className="absolute top-2 right-2 w-64 z-[1000] p-0 space-y-2">
                        <TelemetryCard telemetry={telemetry} heartbeat={heartbeat} />
                        <Card className="bg-transparent p-0 border-none">
                            <Button className="bg-black/80" onClick={toggleFollow}>
                                {isFollowing ? "Stop Following" : "Follow Drone"}
                            </Button>
                        </Card>
                    </div>
                    <div className="absolute bottom-2 left-2 z-[1000] flex flex-row space-x-2">
                        <StatsCard stats={stats}></StatsCard>
                        <BatteryStatusCard battery={battery} />
                    </div>
                </div>
        </div>
    );

    function MapUpdater({ telemetry }: { telemetry: Telemetry | null }) {
        const map = useMap();

        useEffect(() => {
            if (telemetry && isFollowing) {
                map.setView([telemetry.lat, telemetry.lon]);
            }
        }, [telemetry, map, isFollowing]);

        return null;
    }
}

export default App;
